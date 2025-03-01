use sea_orm::FromQueryResult;
use serde::{Deserialize, Serialize};

use super::EditorCurrent;

// ------------------------------------
// 逻辑删除
// ------------------------------------
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Default)]
pub struct ModelLogicDeleteDTO {
    pub id: i64,

    /// 编辑用户
    pub editor: EditorCurrent,
}

// ------------------------------------
// 按关联主键统计
// ------------------------------------
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Default, FromQueryResult)]
pub struct ModelRelationCountDTO {
    /// 关联主键
    pub relation_id: i64,

    /// 统计数量
    pub item_count: i16,
}

// ------------------------------------
// 查看
// ------------------------------------
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Default)]
pub struct ModelViewDTO<T> {
    /// 主键
    pub id: i64,

    /// 编辑用户
    pub editor: EditorCurrent,

    /// 是否启用
    pub enabled: Option<bool>,

    /// 加载关联数据
    pub load_models: Option<Vec<T>>,
}
