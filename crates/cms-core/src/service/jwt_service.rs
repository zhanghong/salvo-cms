use chrono::Duration;
use jsonwebtoken::{self, EncodingKey};
use salvo::prelude::*;
use sea_orm::*;
use uuid::Uuid;

use crate::config::{AppState, JwtConfig};
use crate::domain::dto::{EditorCurrentDTO, JwtClaimsDTO, JwtTokenDTO};
use crate::domain::entity::certificate::{
    ActiveModel as CertificateActiveModel, Column as CertificateColummn,
    Entity as CertificateEntity, Model as CertificateModel,
};
use crate::domain::{HandleResult, handle_ok};
use crate::enums::TokenTypeEnum;
use crate::error::AppError;
use crate::utils::time_utils;

use super::RedisService;

pub struct JwtService {}

impl JwtService {
    /// 用户登录
    pub async fn create(
        user_id: Uuid,
        user_type: &str,
        state: &AppState,
    ) -> HandleResult<CertificateModel> {
        let uuid = uuid::Uuid::new_v4();
        let uuid_string = uuid.to_string();
        let access = Self::generate_access_token(&uuid_string, user_id, user_type).unwrap();
        let refresh = Self::generate_refresh_token(&uuid_string, user_id, user_type).unwrap();
        let now = time_utils::current_time();
        let model = CertificateActiveModel {
            id: Set(uuid.to_owned()),
            user_id: Set(user_id.to_owned()),
            user_type: Set(user_type.to_owned()),
            access_token: Set(access.token_value.to_owned()),
            access_expired_at: Set(time_utils::from_timestamp(access.expired_time)),
            refresh_token: Set(refresh.token_value.to_owned()),
            refresh_expired_at: Set(time_utils::from_timestamp(refresh.expired_time)),
            created_at: Set(now),
            updated_at: Set(now),
            ..Default::default()
        };

        let model: CertificateModel = model.insert(&state.db).await?;
        RedisService::set_jwt_key(&state.redis, &uuid_string, access.expired_time);

        handle_ok(model)
    }

    // 用户刷新 Token
    pub async fn update_by_claims(
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
        let claim_user_id = dto.uuid.to_owned();
        let user_id = Uuid::parse_str(&dto.uuid).unwrap();
        let db = &state.db;
        let model = CertificateEntity::find()
            .filter(CertificateColummn::Id.eq(user_id))
            .one(db)
            .await?
            .unwrap();

        let current_timestamp = time_utils::current_timestamp();
        let refresh_expired_time = time_utils::to_timestamp(model.refresh_expired_at.clone());
        if current_timestamp > refresh_expired_time {
            let err = AppError::Unauthorized;
            return Err(err);
        }

        let user_type = model.user_type.to_owned();
        let user_type = user_type.as_str();
        let mut model: CertificateActiveModel = model.into();
        let access = Self::generate_access_token(&claim_user_id, user_id, user_type).unwrap();
        model.access_token = Set(access.token_value.to_owned());
        model.access_expired_at = Set(time_utils::from_timestamp(access.expired_time));

        if current_timestamp + (3 * 24 * 60 * 60) > refresh_expired_time {
            let refresh = Self::generate_refresh_token(&claim_user_id, user_id, user_type).unwrap();
            model.refresh_token = Set(refresh.token_value.to_owned());
            model.refresh_expired_at = Set(time_utils::from_timestamp(refresh.expired_time));
        }
        model.updated_at = Set(time_utils::current_time());
        let model: CertificateModel = model.update(db).await?;
        RedisService::set_jwt_key(&state.redis, &claim_user_id, access.expired_time);

        handle_ok(model)
    }

    /// 生成 Access Token
    fn generate_access_token(
        uuid: &String,
        user_id: Uuid,
        user_type: &str,
    ) -> HandleResult<JwtTokenDTO> {
        let cfg = JwtConfig::from_env().expect("Failed to load jwt config");
        let secret_bytes = cfg.secret_bytes();
        let days = cfg.get_access_expire_days();
        let now = time_utils::current_time();
        let expired_time = (now + Duration::days(days)).and_utc().timestamp();

        let claims = JwtClaimsDTO {
            uuid: uuid.to_owned(),
            user_id: user_id.to_string(),
            user_type: user_type.to_owned(),
            token_type: TokenTypeEnum::AccessToken.as_value(),
            exp: expired_time,
        };

        let header = jsonwebtoken::Header::default();
        let encode_key = EncodingKey::from_secret(&secret_bytes);

        let dto = JwtTokenDTO {
            token_type: claims.token_type.to_owned(),
            token_value: jsonwebtoken::encode(&header, &claims, &encode_key).unwrap(),
            expired_time,
        };
        handle_ok(dto)
    }

    /// 验证 AccessToken
    pub fn verify_access_token(depot: &mut Depot) -> HandleResult<()> {
        let claims: JwtClaimsDTO;
        match depot.jwt_auth_state() {
            JwtAuthState::Authorized => {
                let data = depot.jwt_auth_data::<JwtClaimsDTO>().unwrap();
                let opt = Some(data.claims.clone());
                claims = opt.ok_or(AppError::Unauthorized).unwrap();
            }
            _ => {
                let err = AppError::Unauthorized;
                return Err(err);
            }
        };

        let state = depot.obtain::<AppState>().unwrap();
        if !RedisService::has_jwt_key(&state.redis, &claims.uuid) {
            let err = AppError::Unauthorized;
            return Err(err);
        }

        let token_type = TokenTypeEnum::form_string(claims.token_type.to_owned());
        match token_type {
            TokenTypeEnum::AccessToken => {}
            _ => {
                let err = AppError::Unauthorized;
                return Err(err);
            }
        }

        let editor: EditorCurrentDTO = claims.into();
        depot.insert("current_editor", editor);

        handle_ok(())
    }

    /// 生成 Refresh Token
    fn generate_refresh_token(
        uuid: &String,
        user_id: Uuid,
        user_type: &str,
    ) -> HandleResult<JwtTokenDTO> {
        let cfg = JwtConfig::from_env().expect("Failed to load jwt config");
        let secret_bytes = cfg.secret_bytes();
        let days = cfg.get_refresh_expire_days();
        let now = time_utils::current_time();
        let expired_time = (now + Duration::days(days)).and_utc().timestamp();

        let claims = JwtClaimsDTO {
            uuid: uuid.to_owned(),
            user_id: user_id.to_string(),
            user_type: user_type.to_owned(),
            token_type: TokenTypeEnum::RefreshToken.as_value(),
            exp: expired_time,
        };

        let header = jsonwebtoken::Header::default();
        let encode_key = EncodingKey::from_secret(&secret_bytes);

        let dto = JwtTokenDTO {
            token_type: claims.token_type.to_owned(),
            token_value: jsonwebtoken::encode(&header, &claims, &encode_key).unwrap(),
            expired_time,
        };

        handle_ok(dto)
    }

    /// 验证 RefreshToken
    pub fn verify_refresh_token(depot: &Depot) -> HandleResult<()> {
        match depot.jwt_auth_state() {
            JwtAuthState::Authorized => {
                let data = depot.jwt_auth_data::<JwtClaimsDTO>().unwrap();
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

    // 删除 Token
    pub async fn delete_by_claims(dto: Option<JwtClaimsDTO>, state: &AppState) -> HandleResult<()> {
        if dto.is_none() {
            return handle_ok(());
        }
        let dto = dto.unwrap();
        let token_type = TokenTypeEnum::form_string(dto.token_type.to_owned());
        match token_type {
            TokenTypeEnum::AccessToken => {}
            _ => {
                let err = AppError::Unauthorized;
                return Err(err);
            }
        }
        let uuid_string = dto.uuid.to_owned();
        RedisService::del_jwt_key(&state.redis, &uuid_string);
        match Uuid::parse_str(&uuid_string) {
            Ok(uuid) => {
                let _ = CertificateEntity::delete_by_id(uuid)
                    .exec(&state.db)
                    .await?;
            }
            Err(_) => {}
        }

        handle_ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::entity::certificate::Model as CertificateModel;
    use crate::enums::EditorTypeEnum;
    use crate::fixture::config::FakerAppState;

    #[tokio::test]
    async fn test_create() {
        let mut state = FakerAppState::init().await;
        let user_id = Uuid::new_v4();
        let type_str = EditorTypeEnum::Admin.string_value();
        let user_type = type_str.as_str();
        let mock_result = MockExecResult {
            last_insert_id: 1,
            rows_affected: 1,
        };
        let insert_uuid = Uuid::new_v4();
        let cert_model = CertificateModel {
            id: insert_uuid.clone(),
            user_id: user_id.clone(),
            user_type: user_type.to_owned(),
            access_token: "".to_owned(),
            access_expired_at: time_utils::current_time(),
            refresh_token: "".to_owned(),
            refresh_expired_at: time_utils::current_time(),
            created_at: time_utils::current_time(),
            updated_at: time_utils::current_time(),
        };
        state.db = MockDatabase::new(DatabaseBackend::Postgres)
            .append_query_results([vec![cert_model]])
            .append_exec_results([mock_result])
            .into_connection();

        let res = JwtService::create(user_id, user_type, &state).await;
        assert!(res.is_ok());
        let model = res.unwrap();
        assert_eq!(model.id, insert_uuid);
        assert_eq!(model.user_id, user_id.clone());
        assert_eq!(model.user_type, user_type.to_owned());
    }

    #[tokio::test]
    async fn test_update_by_claims() {
        let state = FakerAppState::init().await;
        let dto: Option<JwtClaimsDTO> = None;
        let res = JwtService::update_by_claims(dto, &state).await;
        assert!(res.is_err());
        let err = res.unwrap_err();
        assert_eq!(err, AppError::Unauthorized);
    }
}
