use serde::{Deserialize, Serialize};

use cms_core::enums::EditorTypeEnum;

use crate::enums::UserLoadEnum;

// ------------------------------------
// 查看用户
// ------------------------------------
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Default)]
pub struct UserViewDTO {
    /// 主键
    pub id: i64,

    /// 当前密码
    pub user_types: Option<EditorTypeEnum>,

    /// 是否启用
    pub enabled: Option<bool>,

    /// 加载关联数据
    pub load_models: Option<Vec<UserLoadEnum>>,
}
