use serde::{Deserialize, Serialize};

use crate::enums::PrimaryIdEnum;

use super::EditorCurrentDTO;

/// Model Logic Delete DTO
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Default)]
pub struct ModelLogicDeleteDTO {
    pub id: PrimaryIdEnum,

    /// 编辑用户
    pub editor: EditorCurrentDTO,
}
