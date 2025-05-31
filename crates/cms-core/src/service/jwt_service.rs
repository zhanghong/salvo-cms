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
        dto: Option<&JwtClaimsDTO>,
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
        let claim_user_id = dto.user_id.to_owned();
        let user_id = Uuid::parse_str(&dto.uuid).unwrap();
        let db = &state.db;
        let opt = CertificateEntity::find()
            .filter(CertificateColummn::Id.eq(user_id))
            .one(db)
            .await?;
        if opt.is_none() {
            let err = AppError::TokenNotFound;
            return Err(err);
        }
        let model = opt.unwrap();

        let current_timestamp = time_utils::current_timestamp();
        let refresh_expired_time = time_utils::to_timestamp(&model.refresh_expired_at);
        if current_timestamp > refresh_expired_time {
            println!("current_time: {}, refresh expired time: {}", current_timestamp, refresh_expired_time);
            let err = AppError::TokenExpired;
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
    use std::vec;

    use super::*;
    use crate::domain::entity::certificate::Model as CertificateModel;
    use crate::enums::EditorTypeEnum;
    use crate::fixture::config::FakerAppState;

    
    fn cert_table_field_str() -> &'static str {
        r#""id", "user_type", "user_id", "access_token", "access_expired_at", "refresh_token", "refresh_expired_at", "created_at", "updated_at""#
    }

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
        let current_time = time_utils::current_time();
        let cert_model = CertificateModel {
            id: insert_uuid.clone(),
            user_id: user_id.clone(),
            user_type: user_type.to_owned(),
            access_token: "access_token".to_owned(),
            access_expired_at: current_time.clone(),
            refresh_token: "refresh_token".to_owned(),
            refresh_expired_at: current_time.clone(),
            created_at: current_time.clone(),
            updated_at: current_time.clone(),
        };
        state.db = MockDatabase::new(DatabaseBackend::Postgres)
            .append_query_results([vec![cert_model.clone()]])
            .append_exec_results([mock_result])
            .into_connection();

        let res = JwtService::create(user_id, user_type, &state).await;
        assert!(res.is_ok());
        let model = res.unwrap();
        assert_eq!(model.id, cert_model.id);
        assert_eq!(model.user_id, cert_model.user_id);
        assert_eq!(model.user_type, cert_model.user_type);
        let logs = state.db.into_transaction_log();
        let log = logs[0].clone();
        let statements = log.statements();
        assert_eq!(statements.len(), 1);
        let statement = statements[0].clone();
        let table_fields = cert_table_field_str();
        let sql_text = format!(
            r#"INSERT INTO "auth_certificates" ({}) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9) RETURNING {}"#,
            table_fields, table_fields
        );
        let model_values: Vec<Value> = vec![user_type.into(), user_id.into()];
        let state_values = statement.values.unwrap().0;
        let insert_values: Vec<Value> = vec![state_values[1].clone(), state_values[2].clone()];
        assert_eq!(model_values, insert_values);
        assert_eq!(statement.sql, sql_text);
    }

    #[tokio::test]
    async fn test_update_by_claims_fail() {
        let mut state = FakerAppState::init().await;
        let res = JwtService::update_by_claims(None, &state).await;
        assert!(res.is_err());
        let err = res.unwrap_err();
        assert_eq!(err, AppError::Unauthorized);

        let dto_uuid = Uuid::new_v4();   
        let dto_user_id =  Uuid::new_v4();
        let current_timestamp = time_utils::current_timestamp();
        let mut dto = JwtClaimsDTO{
            uuid: dto_uuid.to_string(),
             user_id: dto_user_id.to_string(),
             user_type: EditorTypeEnum::Admin.string_value(),
             token_type: TokenTypeEnum::None.as_value(),
             exp: current_timestamp,
        };
        let res = JwtService::update_by_claims(Some(&dto), &state).await;
        assert!(res.is_err());
        let err = res.unwrap_err();
        assert_eq!(err, AppError::Unauthorized);

        dto.token_type = TokenTypeEnum::AccessToken.as_value();
        let res = JwtService::update_by_claims(Some(&dto), &state).await;
        assert!(res.is_err());
        let err = res.unwrap_err();
        assert_eq!(err, AppError::Unauthorized);

        let cert_model = CertificateModel {
            user_id: dto_user_id.clone(),
            refresh_expired_at: time_utils::current_time() + Duration::hours(1),
            ..Default::default()
        };
        dto.token_type = TokenTypeEnum::RefreshToken.as_value();
        state.db = MockDatabase::new(DatabaseBackend::Postgres)
            .append_query_results([
                // uuid not exists in db
                Vec::<CertificateModel>::new(),
                vec![CertificateModel{
                    refresh_expired_at: time_utils::current_time() - Duration::minutes(1),
                    ..cert_model.clone()
                }],
                vec![CertificateModel{
                    refresh_expired_at: time_utils::current_time() + Duration::minutes(3),
                    ..cert_model.clone()
                }]
            ])
            .into_connection();
        // uuid not exists in db
        let res = JwtService::update_by_claims(Some(&dto), &state).await;
        assert!(res.is_err());
        let err = res.unwrap_err();
        assert_eq!(err, AppError::TokenNotFound);
        // refresh token expired
        let res = JwtService::update_by_claims(Some(&dto), &state).await;
        assert!(res.is_err());
        let err = res.unwrap_err();
        assert_eq!(err, AppError::TokenExpired);
    }

    #[tokio::test]
    async fn test_update_by_claims_ok() {
        let mut state = FakerAppState::init().await;
        let dto_uuid = Uuid::new_v4();   
        let dto_user_id =  Uuid::new_v4();
        let current_timestamp = time_utils::current_timestamp();
        let dto = JwtClaimsDTO{
            uuid: dto_uuid.to_string(),
             user_id: dto_user_id.to_string(),
             user_type: EditorTypeEnum::Admin.string_value(),
             token_type: TokenTypeEnum::RefreshToken.as_value(),
             exp: current_timestamp,
        };
        let cert_model = CertificateModel {
            user_id: dto_user_id.clone(),
            refresh_expired_at: time_utils::current_time() + Duration::minutes(1),
            ..Default::default()
        };
        state.db = MockDatabase::new(DatabaseBackend::Postgres)
            .append_query_results([
                vec![cert_model.clone()],
                vec![cert_model.clone()],
            ])
            .append_exec_results([
                MockExecResult {
                    last_insert_id: 1,
                    rows_affected: 1,
                },
            ])
            .into_connection();
        // refresh token is ok
        let res = JwtService::update_by_claims(Some(&dto), &state).await;
        assert!(res.is_ok());
        let logs = state.db.into_transaction_log();
        let table_fields = cert_table_field_str();
        let update_sql = format!(
            r#"UPDATE "auth_certificates" SET "access_token" = $1, "access_expired_at" = $2, "refresh_token" = $3, "refresh_expired_at" = $4, "updated_at" = $5 WHERE "auth_certificates"."id" = $6 RETURNING {}"#,
            table_fields
        );
        let update_log = logs[1].clone();
        let statements = update_log.statements();
        let update_statement = statements[0].clone();
        assert_eq!(update_statement.sql, update_sql);
        assert_eq!(logs.len(), 2);
    }
}
