use sea_orm::FromQueryResult;
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
