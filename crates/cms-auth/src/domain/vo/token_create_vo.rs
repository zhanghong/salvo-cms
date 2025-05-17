use salvo::oapi::ToSchema;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// 登录 Token VO
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Default, ToSchema)]
#[salvo(schema(name = "Auth::Token::TokenCreateVO"))]
pub struct TokenCreateVO {
    /// 用户ID
    #[salvo(schema(required = true, nullable = false, value_type = KnownFormat::Uuid, example = "00000000-0000-0000-0000-000000000000"))]
    pub user_id: Uuid,

    /// 用户名
    #[salvo(schema(required = true, nullable = false, value_type = String, max_length=30, example = "zhangsan"))]
    pub username: String,

    /// 用户昵称
    #[salvo(schema(required = true, nullable = false, value_type = String, max_length=30, example = "张三"))]
    pub nickname: String,

    /// 用户头像
    #[salvo(schema(required = true, nullable = false, value_type = KnownFormat::Url, example = "https://www.baidu.com/logo.png"))]
    pub avatar: String,

    /// 用户角色
    #[salvo(schema(required = true, nullable = false, example = json!(vec!["admin", "member"])))]
    pub roles: Vec<String>,

    /// 用户按钮级别权限
    #[salvo(schema(required = true, nullable = false, example = json!(vec!["create_user", "create_product"])))]
    pub permissions: Vec<String>,

    /// Access Token
    #[salvo(schema(required = true, nullable = false, value_type = String))]
    pub access_token: String,

    /// Access Token 过期时间
    #[salvo(schema(required = true, nullable = false, value_type = KnownFormat::DateTime, example = "2022-01-01 00:00:00"))]
    pub access_expired: String,

    /// Refresh Token
    #[salvo(schema(required = true, nullable = false, value_type = String))]
    pub refresh_token: String,

    /// Refresh Token 过期时间
    #[salvo(schema(required = true, nullable = false, value_type = KnownFormat::DateTime, example = "2022-01-01 00:00:00", format = "yyyy-MM-dd HH:mm:ss"))]
    pub refresh_expired: String,
}
