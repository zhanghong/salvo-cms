use salvo::oapi::ToSchema;
use serde::{Deserialize, Serialize};

// ------------------------------------
// JWT 登录或刷新 Token VO
// ------------------------------------
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Default, ToSchema)]
#[salvo(schema(name = "Core::Auth::JwtLoginVO"))]
pub struct JwtLoginVO {
    /// Access Token
    #[salvo(schema(required = true, nullable = false))]
    pub access_token: String,

    /// Access Token 过期时间
    #[salvo(schema(required = true, nullable = false, example = 3600))]
    pub access_expired: i64,

    /// Refresh Token
    #[salvo(schema(required = true, nullable = false))]
    pub refresh_token: String,

    /// Refresh Token 过期时间
    #[salvo(schema(required = true, nullable = false, example = 86400))]
    pub refresh_expired: i64,
}
