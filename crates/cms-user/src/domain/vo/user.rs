use salvo::oapi::ToSchema;
use serde::{Deserialize, Serialize};

use cms_core::domain::SelectOptionItem;

use crate::domain::entity::user::Model;
use crate::enums::{GenderEnum, UserTypeEnum};

use super::DetailVO;

// ------------------------------------
// 创建/更新用户
// ------------------------------------
#[derive(Deserialize, Serialize, Debug, Clone, Default, ToSchema)]
pub struct UserFormOptionVO {
    /// 性别选项
    pub genders: Option<Vec<SelectOptionItem>>,

    /// 用户类型选项
    pub types: Option<Vec<SelectOptionItem>>,
}

// ------------------------------------
// 用户详情
// ------------------------------------
// Service 层创建/更新用户使用的结构体
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Default, ToSchema)]
pub struct UserItemVO {
    /// 主键
    pub id: i64,

    /// NO
    pub no: String,

    /// 登录名
    pub name: String,

    /// 真实姓名
    #[serde(skip_serializing_if = "Option::is_none")]
    pub realname: Option<String>,

    /// 昵称
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nickname: Option<String>,

    /// 角色类型
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_types: Option<Vec<UserTypeEnum>>,

    /// 性别
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gender: Option<i16>,

    /// 性别
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gender_title: Option<GenderEnum>,

    /// 手机号码
    pub phone: Option<String>,

    /// 手机号码
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone_shown: Option<String>,

    /// 头像URL
    pub avatar_url: String,

    /// 邮箱
    pub email: Option<String>,

    /// 注册来源
    pub data_source_id: Option<i64>,

    /// 是否认证
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_authed: Option<bool>,

    /// 是否启用
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_enabled: Option<bool>,

    /// 是否测试账号
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_test: Option<bool>,

    /// 创建时间
    pub created_time: String,

    /// 更新时间
    pub updated_time: String,

    /// 详情信息
    #[serde(skip_serializing_if = "Option::is_none")]
    pub detail: Option<DetailVO>,
}

impl From<Model> for UserItemVO {
    fn from(model: Model) -> Self {
        let avatar_url = model.avatar_url();
        let created_time = model.created_time();
        let updated_time = model.updated_time();
        Self {
            id: model.id,
            no: model.no,
            name: model.name,
            realname: Some(model.realname.to_owned()),
            nickname: Some(model.nickname.to_owned()),
            gender: Some(model.gender),
            phone: Some(model.phone.to_owned()),
            avatar_url: avatar_url,
            email: Some(model.email.to_owned()),
            data_source_id: Some(model.data_source_id),
            is_authed: Some(model.is_authed),
            is_enabled: Some(model.is_enabled),
            is_test: Some(model.is_test),
            created_time: created_time,
            updated_time: updated_time,
            ..Default::default()
        }
    }
}
