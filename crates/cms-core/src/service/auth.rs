use chrono::Duration;
use jsonwebtoken::{self, EncodingKey};
use salvo::prelude::*;
use salvo::Depot;

use crate::config::JwtConfig;
use crate::domain::dto::JwtClaimsDTO;
use crate::domain::{handle_ok, HandleResult};
use crate::enums::TokenTypeEnum;
use crate::error::AppError;
use crate::utils::time;

pub struct AuthService {}

impl AuthService {
    pub fn generate_access_token(user_id: i64, user_type: &str) -> HandleResult<String> {
        let cfg = JwtConfig::from_env().expect("Failed to load jwt config");
        let secret_bytes = cfg.access_secret_bytes();
        let days = cfg.get_access_expire_days();
        let now = time::current_time();
        let expired = (now + Duration::days(days)).and_utc().timestamp();

        let claims = JwtClaimsDTO {
            user_id: user_id,
            user_type: user_type.to_owned(),
            token_type: TokenTypeEnum::AccessToken.as_value(),
            exp: expired,
        };

        let header = jsonwebtoken::Header::default();
        let encode_key = EncodingKey::from_secret(&secret_bytes);
        let token = jsonwebtoken::encode(&header, &claims, &encode_key).unwrap();
        handle_ok(token)
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
                        let err = AppError::BadRequest("Invalid token".to_string());
                        return Err(err);
                    }
                }
            }
            JwtAuthState::Unauthorized => {
                let err = AppError::Unauthorized;
                return Err(err);
            }
            JwtAuthState::Forbidden => {
                let err = AppError::Forbidden;
                return Err(err);
            }
        };

        handle_ok(())
    }
}
