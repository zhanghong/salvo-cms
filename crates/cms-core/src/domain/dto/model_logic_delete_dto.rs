use serde::{Deserialize, Serialize};

use super::EditorCurrentDTO;

// ------------------------------------
// 逻辑删除
// ------------------------------------
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Default)]
pub struct ModelLogicDeleteDTO {
    pub id: i64,

    /// 编辑用户
    pub editor: EditorCurrentDTO,
}
