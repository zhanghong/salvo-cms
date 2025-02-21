use salvo::oapi::ToSchema;
use serde::{Deserialize, Serialize};

// ------------------------------------
// 登录 Token
// ------------------------------------
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Default, ToSchema)]
pub struct LoginTokenCreateVO {
    /// 用户ID
    pub user_id: i64,

    /// 用户名
    pub username: String,

    /// 用户昵称
    pub nickname: String,

    /// 用户头像
    pub avatar: String,

    /// 用户角色
    pub roles: Vec<String>,

    /// 用户按钮级别权限
    pub permissions: Vec<String>,

    /// Access Token
    pub access_token: String,

    /// Access Token 过期时间
    pub access_expired: String,

    /// Refresh Token
    pub refresh_token: String,

    /// Refresh Token 过期时间
    pub refresh_expired: String,
}

// ------------------------------------
// 登录 Token
// ------------------------------------
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Default, ToSchema)]
pub struct LoginTokenUpdateVO {
    /// Access Token
    pub access_token: String,

    /// Access Token 过期时间
    pub access_expired: String,

    /// Refresh Token
    pub refresh_token: String,

    /// Refresh Token 过期时间
    pub refresh_expired: String,
}
