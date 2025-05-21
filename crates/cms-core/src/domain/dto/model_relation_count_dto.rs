// use sea_orm::FromQueryResult;
use serde::{Deserialize, Serialize};

use crate::enums::PrimaryIdEnum;

/// Model Reloation Count DTO
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Default)]
pub struct ModelRelationCountDTO {
    /// 关联主键
    pub relation_id: PrimaryIdEnum,

    /// 统计数量
    pub item_count: i16,
}
