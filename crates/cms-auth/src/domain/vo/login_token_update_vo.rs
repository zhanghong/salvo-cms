use salvo::oapi::ToSchema;
use serde::{Deserialize, Serialize};

/// 刷新 Token VO
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Default, ToSchema)]
#[salvo(schema(name = "权鉴模块/登录/刷新 Token VO"))]
pub struct LoginTokenUpdateVO {
    /// Access Token
    #[salvo(schema(required = true, nullable = false, value_type = String))]
    pub access_token: String,

    /// Access Token 过期时间
    #[salvo(schema(required = true, nullable = false, value_type = String, example = "2022-01-01 00:00:00", format = "yyyy-MM-dd HH:mm:ss"))]
    pub access_expired: String,

    /// Refresh Token
    #[salvo(schema(required = true, nullable = false, value_type = String))]
    pub refresh_token: String,

    /// Refresh Token 过期时间
    #[salvo(schema(required = true, nullable = false, value_type = String, example = "2022-01-01 00:00:00", format = "yyyy-MM-dd HH:mm:ss"))]
    pub refresh_expired: String,
}
