use salvo::oapi::ToSchema;
use serde::{Deserialize, Serialize};

use cms_core::domain::model::SelectOptionModel;

// ------------------------------------
// 创建/更新用户表单 VO
// ------------------------------------
#[derive(Deserialize, Serialize, Debug, Clone, Default, ToSchema)]
pub struct UserFormOptionVO {
    /// 性别选项
    pub genders: Option<Vec<SelectOptionModel>>,

    /// 用户类型选项
    pub types: Option<Vec<SelectOptionModel>>,
}
