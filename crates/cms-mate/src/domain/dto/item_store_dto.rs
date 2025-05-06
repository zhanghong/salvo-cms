use serde::{Deserialize, Serialize};

use cms_core::domain::dto::EditorCurrent;

use crate::domain::form::ItemStoreForm;

/// Item Store DTO
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Default)]
pub struct ItemStoreDTO {
    /// 主键
    pub id: i64,

    /// 编辑用户
    pub editor: EditorCurrent,

    /// 模块ID
    pub app_id: Option<i64>,

    /// 类型ID
    pub kind_id: Option<i64>,

    /// 名称
    pub name: Option<String>,

    /// 标题
    pub title: Option<String>,

    /// 描述
    pub description: Option<String>,

    /// 介绍
    pub introduction: Option<String>,

    /// 介绍
    pub icon: Option<String>,

    /// PC端封面图片
    pub pc_detail_path: Option<String>,

    /// 手机端封面图片
    pub wap_detail_path: Option<String>,

    /// 父级ID
    pub parent_id: Option<i64>,

    /// 版本号
    pub version_no: Option<i32>,

    /// 排序编号
    pub sort: Option<i16>,

    /// 是否启用
    pub is_enabled: Option<bool>,
}

impl ItemStoreDTO {
    fn by_store_form(model: &ItemStoreForm) -> Self {
        Self {
            app_id: model.app_id.clone(),
            kind_id: model.kind_id.clone(),
            name: model.name.clone(),
            title: model.title.clone(),
            description: model.description.clone(),
            introduction: model.introduction.clone(),
            icon: model.icon.clone(),
            pc_detail_path: model.pc_detail_path.clone(),
            wap_detail_path: model.wap_detail_path.clone(),
            parent_id: model.parent_id.clone(),
            version_no: model.version_no.clone(),
            sort: model.sort.clone(),
            is_enabled: model.is_enabled.clone(),
            ..Default::default()
        }
    }
}

impl From<ItemStoreForm> for ItemStoreDTO {
    fn from(model: ItemStoreForm) -> Self {
        Self::by_store_form(&model)
    }
}

impl From<&ItemStoreForm> for ItemStoreDTO {
    fn from(model: &ItemStoreForm) -> Self {
        Self::by_store_form(model)
    }
}
