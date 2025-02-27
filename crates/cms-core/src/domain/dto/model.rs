use serde::{Deserialize, Serialize};

use crate::enums::EditorTypeEnum;

// ------------------------------------
// 逻辑删除
// ------------------------------------
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Default)]
pub struct ModelLogicDeleteDTO {
    pub id: i64,

    /// 编辑用户类型
    pub editor_type: EditorTypeEnum,

    /// 编辑用户ID
    pub editor_id: i64,
}
