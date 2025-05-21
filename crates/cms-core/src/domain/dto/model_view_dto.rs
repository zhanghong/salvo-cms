use serde::{Deserialize, Serialize};

use crate::enums::PrimaryIdEnum;

use super::EditorCurrentDTO;

/// Model View DTO
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Default)]
pub struct ModelViewDTO<T> {
    /// 主键
    pub id: PrimaryIdEnum,

    /// 编辑用户
    pub editor: EditorCurrentDTO,

    /// 是否启用
    pub enabled: Option<bool>,

    /// 加载关联数据
    pub load_models: Option<Vec<T>>,
}
