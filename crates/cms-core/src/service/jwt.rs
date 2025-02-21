use chrono::Duration;
use jsonwebtoken::{self, EncodingKey};
use salvo::prelude::*;
use salvo::Depot;
use sea_orm::*;

use crate::config::AppState;
use crate::config::JwtConfig;
use crate::domain::dto::{JwtClaimsDTO, JwtTokenDTO};
use crate::domain::entity::certificate::{
    ActiveModel as CertificateActiveModel, Column as CertificateColummn,
    Entity as CertificateEntity, Model as CertificateModel,
};
use crate::domain::{handle_ok, HandleResult};
use crate::enums::TokenTypeEnum;
use crate::error::AppError;
use crate::utils::time;

use super::RedisService;

pub struct JwtService {}

impl JwtService {
    /// 用户登录
    pub async fn user_login(
        user_id: i64,
        user_type: &str,
        state: &AppState,
    ) -> HandleResult<CertificateModel> {
        let uuid: String = uuid::Uuid::new_v4().to_string();
        let access = Self::generate_access_token(&uuid, user_id, user_type).unwrap();
        let refresh = Self::generate_refresh_token(&uuid, user_id, user_type).unwrap();

        let now = time::current_time();
        let model = CertificateActiveModel {
            id: Set(uuid.to_owned()),
            user_id: Set(user_id),
            user_type: Set(user_type.to_owned()),
            access_token: Set(access.token_value.to_owned()),
            access_expired_at: Set(time::from_timestamp(access.expired_time)),
            refresh_token: Set(refresh.token_value.to_owned()),
            refresh_expired_at: Set(time::from_timestamp(refresh.expired_time)),
            created_at: Set(now),
            updated_at: Set(now),
            ..Default::default()
        };
        let _ = CertificateEntity::insert(model.clone())
            .exec(&state.db)
            .await?;
        RedisService::set_jwt_key(&state.redis, &uuid, access.expired_time);

        let model = model.try_into_model().unwrap();
        handle_ok(model)
    }

    // 用户刷新 Token
    pub async fn user_refresh_by_claims(
        dto: Option<JwtClaimsDTO>,
        state: &AppState,
    ) -> HandleResult<CertificateModel> {
        if dto.is_none() {
            let err = AppError::Unauthorized;
            return Err(err);
        }
        let dto = dto.unwrap();
        let token_type = TokenTypeEnum::form_string(dto.token_type.to_owned());
        match token_type {
            TokenTypeEnum::RefreshToken => {}
            _ => {
                let err = AppError::Unauthorized;
                return Err(err);
            }
        }
        let uuid = dto.uuid.to_owned();
        let db = &state.db;
        let model = CertificateEntity::find()
            .filter(CertificateColummn::Id.eq(&uuid))
            .one(db)
            .await?
            .unwrap();

        let now = time::current_timestamp();
        let refresh_expired_time = time::to_timestamp(model.refresh_expired_at.clone());
        if now > refresh_expired_time {
            let err = AppError::Unauthorized;
            return Err(err);
        }

        let user_id = model.user_id;
        let user_type = model.user_type.to_owned();
        let user_type = user_type.as_str();
        let mut model: CertificateActiveModel = model.into();
        let access = Self::generate_access_token(&uuid, user_id, user_type).unwrap();
        model.access_token = Set(access.token_value.to_owned());
        model.access_expired_at = Set(time::from_timestamp(access.expired_time));

        if now + (3 * 24 * 60 * 60) > refresh_expired_time {
            let refresh = Self::generate_refresh_token(&uuid, user_id, user_type).unwrap();
            model.refresh_token = Set(refresh.token_value.to_owned());
            model.refresh_expired_at = Set(time::from_timestamp(refresh.expired_time));
        }
        let _ = CertificateEntity::update_many()
            .set(model.clone())
            .filter(CertificateColummn::Id.eq(uuid.to_owned()))
            .exec(db)
            .await?;

        RedisService::set_jwt_key(&state.redis, &uuid, access.expired_time);

        let model = model.try_into_model().unwrap();
        handle_ok(model)
    }

    fn generate_access_token(
        uuid: &String,
        user_id: i64,
        user_type: &str,
    ) -> HandleResult<JwtTokenDTO> {
        let cfg = JwtConfig::from_env().expect("Failed to load jwt config");
        let secret_bytes = cfg.secret_bytes();
        let days = cfg.get_access_expire_days();
        let now = time::current_time();
        let expired_time = (now + Duration::days(days)).and_utc().timestamp();

        let claims = JwtClaimsDTO {
            uuid: uuid.to_owned(),
            user_id: user_id,
            user_type: user_type.to_owned(),
            token_type: TokenTypeEnum::AccessToken.as_value(),
            exp: expired_time,
        };

        let header = jsonwebtoken::Header::default();
        let encode_key = EncodingKey::from_secret(&secret_bytes);

        let dto = JwtTokenDTO {
            token_type: claims.token_type.to_owned(),
            token_value: jsonwebtoken::encode(&header, &claims, &encode_key).unwrap(),
            expired_time: expired_time,
        };
        handle_ok(dto)
    }

    pub fn verify_access_token(depot: &Depot) -> HandleResult<()> {
        match depot.jwt_auth_state() {
            JwtAuthState::Authorized => {
                let data = depot.jwt_auth_data::<JwtClaimsDTO>().unwrap();
                println!("data: {:#?}", data.claims);
                let token_type = TokenTypeEnum::form_string(data.claims.token_type.to_owned());
                match token_type {
                    TokenTypeEnum::AccessToken => {}
                    _ => {
                        let err = AppError::Unauthorized;
                        return Err(err);
                    }
                }
            }
            _ => {
                let err = AppError::Unauthorized;
                return Err(err);
            }
        };

        handle_ok(())
    }

    fn generate_refresh_token(
        uuid: &String,
        user_id: i64,
        user_type: &str,
    ) -> HandleResult<JwtTokenDTO> {
        let cfg = JwtConfig::from_env().expect("Failed to load jwt config");
        let secret_bytes = cfg.secret_bytes();
        let days = cfg.get_refresh_expire_days();
        let now = time::current_time();
        let expired_time = (now + Duration::days(days)).and_utc().timestamp();

        let claims = JwtClaimsDTO {
            uuid: uuid.to_owned(),
            user_id: user_id,
            user_type: user_type.to_owned(),
            token_type: TokenTypeEnum::RefreshToken.as_value(),
            exp: expired_time,
        };

        let header = jsonwebtoken::Header::default();
        let encode_key = EncodingKey::from_secret(&secret_bytes);

        let dto = JwtTokenDTO {
            token_type: claims.token_type.to_owned(),
            token_value: jsonwebtoken::encode(&header, &claims, &encode_key).unwrap(),
            expired_time: expired_time,
        };

        handle_ok(dto)
    }

    pub fn verify_fresh_token(depot: &Depot) -> HandleResult<()> {
        match depot.jwt_auth_state() {
            JwtAuthState::Authorized => {
                let data = depot.jwt_auth_data::<JwtClaimsDTO>().unwrap();
                println!("data: {:#?}", data.claims);
                let token_type = TokenTypeEnum::form_string(data.claims.token_type.to_owned());
                match token_type {
                    TokenTypeEnum::RefreshToken => {}
                    _ => {
                        let err = AppError::Unauthorized;
                        return Err(err);
                    }
                }
            }
            _ => {
                let err = AppError::Unauthorized;
                return Err(err);
            }
        };

        handle_ok(())
    }
}
