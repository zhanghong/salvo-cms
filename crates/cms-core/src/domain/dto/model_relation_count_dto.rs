use sea_orm::FromQueryResult;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

// ------------------------------------
// 按关联主键统计
// ------------------------------------
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Default, FromQueryResult)]
pub struct ModelRelationCountDTO {
    /// 关联主键
    pub relation_id: Uuid,

    /// 统计数量
    pub item_count: i16,
}
