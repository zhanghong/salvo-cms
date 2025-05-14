use redis_macros::{FromRedisValue, ToRedisArgs};
use salvo::oapi::ToSchema;
use serde::{Deserialize, Serialize};

use cms_core::{domain::vo::EditorLoadVO, enums::ViewModeEnum};

use super::app_load_vo::AppLoadVO;
use super::item_load_vo::ItemLoadVO;
use super::kind_load_vo::KindLoadVO;
use crate::domain::entity::item::Model;

/// Item 主 VO
#[derive(
    Debug, Clone, PartialEq, Default, Deserialize, Serialize, ToSchema, FromRedisValue, ToRedisArgs,
)]
#[salvo(schema(name = "Mate/Item/ItemMasterVO"))]
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
    pub description: String,

    /// 介绍
    #[serde(skip_serializing_if = "Option::is_none")]
    pub introduction: Option<String>,

    /// 图标
    pub icon: String,

    /// PC详情URL
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pc_detail_url: Option<String>,

    /// WAP详情URL
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wap_detail_url: Option<String>,

    /// 父级ID
    pub parent_id: i64,

    /// 级别
    pub level: i32,

    /// 是否目录
    pub is_directory: bool,

    /// 版本号
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version_no: Option<i32>,

    /// 排序编号
    pub sort: i16,

    /// 是否启用
    pub is_enabled: bool,

    /// 是否可以更新
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_update: Option<bool>,

    /// 是否可以删除
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_delete: Option<bool>,

    /// 创建时间
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_time: Option<String>,

    /// 更新时间
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_time: Option<String>,

    /// 编辑用户
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
    pub fn mode_into(view_enum: &ViewModeEnum, model: &Model) -> Self {
        let mut vo = Self {
            id: model.id,
            app_id: model.app_id,
            kind_id: model.kind_id,
            name: model.name.to_owned(),
            title: model.title.to_owned(),
            description: model.description.to_owned(),
            icon: model.icon.to_owned(),
            parent_id: model.parent_id,
            level: model.level,
            is_directory: model.is_directory,
            sort: model.sort,
            ..Default::default()
        };

        match view_enum {
            ViewModeEnum::ManagerList => {
                vo.editor_type = model.editor_type.to_owned();
                vo.editor_id = model.editor_id;
                vo.version_no = model.version_no;
                vo.created_time = model.created_time().clone();
                vo.updated_time = model.updated_time().clone();
            }
            ViewModeEnum::ManagerDetail => {
                vo.editor_type = model.editor_type.to_owned();
                vo.editor_id = model.editor_id;
                vo.version_no = model.version_no;
                vo.introduction = model.introduction.clone();
                vo.pc_detail_url = model.pc_detail_url().clone();
                vo.wap_detail_url = model.wap_detail_url().clone();
                vo.created_time = model.created_time().clone();
                vo.updated_time = model.updated_time().clone();
            }
            _ => {}
        }

        vo
    }
}
