use salvo::oapi::ToSchema;
use serde::{Deserialize, Serialize};

// ------------------------------------
// 分页查询 VO
// ------------------------------------
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Default, ToSchema)]
pub struct EditorVO {
    /// 主键
    pub id: i64,

    /// NO
    pub no: String,

    /// 用户名
    pub name: String,

    /// 手机号码
    pub phone: String,

    /// 邮箱
    pub email: String,

    /// 头像URL
    pub avatar_url: String,
}
