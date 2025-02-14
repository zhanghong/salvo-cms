use salvo::oapi::ToSchema;
use serde::{Deserialize, Serialize};

use cms_core::domain::SelectOptionItem;

// ------------------------------------
// 创建/更新用户
// ------------------------------------
// Service 层创建/更新用户使用的结构体
#[derive(Deserialize, Serialize, Debug, Clone, Default, ToSchema)]
pub struct UserFormOptionVO {
    /// 性别选项
    pub genders: Option<Vec<SelectOptionItem>>,

    /// 用户类型选项
    pub types: Option<Vec<SelectOptionItem>>,
}
