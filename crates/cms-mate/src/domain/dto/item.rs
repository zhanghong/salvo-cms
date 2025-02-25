use serde::{Deserialize, Serialize};

use crate::domain::form::ItemStoreForm;

// ------------------------------------
// 创建/更新用户
// ------------------------------------
// Service 层创建/更新用户使用的结构体
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Default)]
pub struct ItemStoreDTO {
    /// 模块ID
    pub module_id: Option<i64>,

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

    /// 排序编号
    pub sort: Option<i64>,

    /// 是否启用
    pub is_enabled: Option<bool>,
}

impl ItemStoreDTO {
    fn by_store_form(model: &ItemStoreForm) -> Self {
        Self {
            module_id: model.module_id.clone(),
            kind_id: model.kind_id.clone(),
            name: model.name.clone(),
            title: model.title.clone(),
            description: model.description.clone(),
            introduction: model.introduction.clone(),
            icon: model.icon.clone(),
            pc_detail_path: model.pc_detail_path.clone(),
            wap_detail_path: model.wap_detail_path.clone(),
            parent_id: model.parent_id.clone(),
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
