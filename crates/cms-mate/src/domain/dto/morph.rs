use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use cms_core::enums::EditorTypeEnum;

use crate::{
    domain::{form::MorphInstanceStoreForm, query::MorphInstanceQuery},
    enums::MorphLoadEnum,
};

// ------------------------------------
// 创建/更新用户
// ------------------------------------
// Service 层创建/更新用户使用的结构体
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Default)]
pub struct MorphInstanceStoreDTO {
    /// 编辑用户类型
    pub editor_type: EditorTypeEnum,

    /// 编辑用户ID
    pub editor_id: i64,

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

// ------------------------------------
// 查询
// ------------------------------------
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Default)]
pub struct MorphInstanceQueryDTO {
    /// 实例类型
    pub instance_type: Option<String>,

    /// 实例ID
    pub instance_id: Option<i64>,

    /// 类型列表
    pub kind_names: Option<Vec<String>>,

    /// 加载关联数据
    pub load_models: Option<Vec<MorphLoadEnum>>,
}

impl MorphInstanceQueryDTO {
    fn from_inner(model: &MorphInstanceQuery) -> Self {
        Self {
            instance_type: model.instance_type.clone(),
            instance_id: model.instance_id.clone(),
            kind_names: model.kind_names.clone(),
            ..Default::default()
        }
    }
}

impl From<MorphInstanceQuery> for MorphInstanceQueryDTO {
    fn from(model: MorphInstanceQuery) -> Self {
        Self::from_inner(&model)
    }
}

impl From<&MorphInstanceQuery> for MorphInstanceQueryDTO {
    fn from(model: &MorphInstanceQuery) -> Self {
        Self::from_inner(model)
    }
}
