use serde::{Deserialize, Serialize};

use super::EditorCurrentDTO;

// ------------------------------------
// 查看
// ------------------------------------
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Default)]
pub struct ModelViewDTO<T> {
    /// 主键
    pub id: i64,

    /// 编辑用户
    pub editor: EditorCurrentDTO,

    /// 是否启用
    pub enabled: Option<bool>,

    /// 加载关联数据
    pub load_models: Option<Vec<T>>,
}
