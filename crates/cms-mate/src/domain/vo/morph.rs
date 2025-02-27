use cms_core::domain::vo::EditorLoadVO;
use salvo::oapi::ToSchema;
use serde::{Deserialize, Serialize};

use super::ItemLoadVO;
use crate::domain::entity::morph::Model;

// ------------------------------------
// 详情视图
// ------------------------------------
// Service 层创建/更新用户使用的结构体
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Default, ToSchema)]
pub struct MorphInstanceVO {
    /// 主键
    pub id: i64,

    /// 编辑用户类型
    #[serde(skip_serializing)]
    pub editor_type: String,

    /// 编辑用户ID
    #[serde(skip_serializing)]
    pub editor_id: i64,

    /// 模块ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub app_id: Option<i64>,

    /// 类型ID
    pub kind_id: i64,

    /// 类型ID
    pub item_id: i64,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub editor: Option<EditorLoadVO>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub item: Option<ItemLoadVO>,
}

impl MorphInstanceVO {
    fn from_model_inner(model: &Model) -> Self {
        Self {
            id: model.id,
            editor_type: model.editor_type.to_owned(),
            editor_id: model.editor_id,
            app_id: Some(model.app_id),
            kind_id: model.kind_id,
            item_id: model.item_id,
            ..Default::default()
        }
    }
}

impl From<Model> for MorphInstanceVO {
    fn from(model: Model) -> Self {
        Self::from_model_inner(&model)
    }
}

impl From<&Model> for MorphInstanceVO {
    fn from(model: &Model) -> Self {
        Self::from_model_inner(model)
    }
}
