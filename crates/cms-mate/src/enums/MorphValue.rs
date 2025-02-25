use salvo::oapi::ToSchema;
use serde::{Deserialize, Serialize};

use crate::domain::{SelectOptionItem, SelectValueEnum};

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, ToSchema)]
pub enum MorphValueEnum {
    Single(u64),
    Multiple(Vec<u64>),
}

impl MorphValueEnum {}
