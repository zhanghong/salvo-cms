use cms_core::domain::vo::EditorLoadVO;
use salvo::oapi::ToSchema;
use serde::{Deserialize, Serialize};

use cms_core::domain::SelectOptionItem;

use super::app::AppLoadVO;
use super::kind::KindLoadVO;
use crate::domain::entity::item::Model;

// ------------------------------------
// 创建/更新表单选项
// ------------------------------------
#[derive(Deserialize, Serialize, Debug, Clone, Default, ToSchema)]
pub struct ItemFormOptionVO {
    /// App 选项
    #[serde(skip_serializing_if = "Option::is_none")]
    pub apps: Option<Vec<SelectOptionItem>>,

    /// 类型
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kinds: Option<Vec<SelectOptionItem>>,

    /// 父级
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parents: Option<Vec<SelectOptionItem>>,

    /// 启用状态
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enables: Option<Vec<SelectOptionItem>>,
}

// ------------------------------------
// 查询表单选项
// ------------------------------------
#[derive(Deserialize, Serialize, Debug, Clone, Default, ToSchema)]
pub struct ItemQueryOptionVO {
    /// App 选项
    #[serde(skip_serializing_if = "Option::is_none")]
    pub apps: Option<Vec<SelectOptionItem>>,

    /// 类型
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kinds: Option<Vec<SelectOptionItem>>,

    /// 父级
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parents: Option<Vec<SelectOptionItem>>,

    /// 启用状态
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enables: Option<Vec<SelectOptionItem>>,
}

// ------------------------------------
// 用户详情
// ------------------------------------
// Service 层创建/更新用户使用的结构体
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Default, ToSchema)]
pub struct ItemMasterVO {
    /// 主键
    pub id: i64,

    /// 编辑用户类型
    #[serde(skip_serializing)]
    pub editor_type: String,

    /// 编辑用户ID
    #[serde(skip_serializing)]
    pub editor_id: i64,

    /// App ID
    pub app_id: i64,

    /// 类型ID
    pub kind_id: i64,

    /// 名称
    pub name: String,

    /// 标题
    pub title: String,

    /// 描述
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    /// 介绍
    #[serde(skip_serializing_if = "Option::is_none")]
    pub introduction: Option<String>,

    /// 图标
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon: Option<String>,

    /// PC详情URL
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pc_detail_url: Option<String>,

    /// WAP详情URL
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wap_detail_url: Option<String>,

    /// 父级ID
    pub parent_id: i64,

    /// 级别
    #[serde(skip_serializing_if = "Option::is_none")]
    pub level: Option<i32>,

    /// 是否目录
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_directory: Option<bool>,

    /// 版本号
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version_no: Option<i32>,

    /// 排序编号
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort: Option<i16>,

    /// 是否启用
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_enabled: Option<bool>,

    /// 是否可以更新
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_update: Option<bool>,

    /// 是否可以删除
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_delete: Option<bool>,

    /// 创建时间
    pub created_time: String,

    /// 更新时间
    pub updated_time: String,

    /// 详情信息
    #[serde(skip_serializing_if = "Option::is_none")]
    pub editor: Option<EditorLoadVO>,

    /// 模块
    #[serde(skip_serializing_if = "Option::is_none")]
    pub app: Option<AppLoadVO>,

    /// 类型
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kind: Option<KindLoadVO>,

    /// 父级
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent: Option<ItemLoadVO>,

    /// 子级
    #[serde(skip_serializing_if = "Option::is_none")]
    pub children: Option<Vec<ItemLoadVO>>,
}

impl ItemMasterVO {
    fn from_model_inner(model: &Model) -> Self {
        let pc_detail_url = model.pc_detail_url();
        let wap_detail_url = model.wap_detail_url();
        let created_time = model.created_time();
        let updated_time = model.updated_time();

        Self {
            id: model.id,
            editor_type: model.editor_type.to_owned(),
            editor_id: model.editor_id,
            app_id: model.app_id,
            kind_id: model.kind_id,
            name: model.name.to_owned(),
            title: model.title.to_owned(),
            description: Some(model.description.to_owned()),
            introduction: model.introduction.to_owned(),
            icon: Some(model.icon.to_owned()),
            pc_detail_url: Some(pc_detail_url),
            wap_detail_url: Some(wap_detail_url),
            parent_id: model.parent_id,
            level: Some(model.level),
            is_directory: Some(model.is_directory),
            version_no: Some(model.version_no),
            sort: Some(model.sort),
            is_enabled: Some(model.is_enabled),
            created_time,
            updated_time,
            ..Default::default()
        }
    }
}

impl From<Model> for ItemMasterVO {
    fn from(model: Model) -> Self {
        Self::from_model_inner(&model)
    }
}

impl From<&Model> for ItemMasterVO {
    fn from(model: &Model) -> Self {
        Self::from_model_inner(model)
    }
}

// ------------------------------------
// 关联视图
// ------------------------------------
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Default, ToSchema)]
pub struct ItemLoadVO {
    /// 主键
    pub id: i64,

    /// App ID
    #[serde(skip_serializing)]
    pub app_id: i64,

    /// 类型ID
    #[serde(skip_serializing)]
    pub kind_id: i64,

    /// 名称
    pub name: String,

    /// 标题
    pub title: String,

    /// 图标
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon: Option<String>,

    /// PC详情URL
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pc_detail_url: Option<String>,

    /// WAP详情URL
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wap_detail_url: Option<String>,

    /// 模块
    #[serde(skip_serializing_if = "Option::is_none")]
    pub app: Option<AppLoadVO>,

    /// 类型
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kind: Option<KindLoadVO>,
}

impl ItemLoadVO {
    fn from_model_inner(model: &Model) -> Self {
        Self {
            id: model.id,
            app_id: model.app_id,
            kind_id: model.kind_id,
            name: model.name.to_owned(),
            title: model.title.to_owned(),
            ..Default::default()
        }
    }
}

impl From<Model> for ItemLoadVO {
    fn from(model: Model) -> Self {
        Self::from_model_inner(&model)
    }
}

impl From<&Model> for ItemLoadVO {
    fn from(model: &Model) -> Self {
        Self::from_model_inner(model)
    }
}
