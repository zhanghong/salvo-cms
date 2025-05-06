use salvo::oapi::ToSchema;
use serde::{Deserialize, Serialize};

/// Morph 关联值类型
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, ToSchema)]
pub enum MorphValueEnum {
    Single(u64),
    Multiple(Vec<u64>),
}

impl MorphValueEnum {}
