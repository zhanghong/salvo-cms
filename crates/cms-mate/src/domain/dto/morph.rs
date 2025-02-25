use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::domain::form::MorphInstanceStoreForm;

// ------------------------------------
// 创建/更新用户
// ------------------------------------
// Service 层创建/更新用户使用的结构体
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Default)]
pub struct MorphInstanceStoreDTO {
    /// 名称
    pub instance_type: Option<String>,

    /// 实例ID
    pub instance_id: Option<i64>,

    /// 关联Item列表
    pub items: Option<HashMap<String, String>>,
}

impl MorphInstanceStoreDTO {
    fn by_store_form(model: &MorphInstanceStoreForm) -> Self {
        Self {
            instance_type: model.instance_type.clone(),
            instance_id: model.instance_id.clone(),
            ..Default::default()
        }
    }
}

impl From<MorphInstanceStoreForm> for MorphInstanceStoreDTO {
    fn from(model: MorphInstanceStoreForm) -> Self {
        Self::by_store_form(&model)
    }
}

impl From<&MorphInstanceStoreForm> for MorphInstanceStoreDTO {
    fn from(model: &MorphInstanceStoreForm) -> Self {
        Self::by_store_form(model)
    }
}
