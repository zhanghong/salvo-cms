use serde::{Deserialize, Serialize};

use cms_core::domain::dto::EditorCurrent;

use crate::{domain::query::MorphInstanceQuery, enums::MorphLoadEnum};

/// Morph 实例查询
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Default)]
pub struct MorphInstanceQueryDTO {
    /// 编辑用户
    pub editor: EditorCurrent,

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
