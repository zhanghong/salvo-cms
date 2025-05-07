use serde::{Deserialize, Serialize};

use cms_core::enums::EditorTypeEnum;

use crate::{
    domain::form::{UserCreateForm, UserUpdateForm},
    enums::GenderEnum,
};

use super::DetailStoreDTO;

// ------------------------------------
// 创建/更新用户
// ------------------------------------
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Default)]
pub struct UserStoreDTO {
    /// 主键
    pub id: Option<i64>,

    /// NO
    pub no: Option<String>,

    /// 登录名
    pub name: Option<String>,

    /// 真实姓名
    pub real_name: Option<String>,

    /// 昵称
    pub nickname: Option<String>,

    /// 角色类型
    pub types_list: Option<Vec<EditorTypeEnum>>,

    /// 性别
    pub gender: Option<GenderEnum>,

    /// 手机号码
    pub phone: Option<String>,

    /// 头像URL
    pub avatar_path: Option<String>,

    /// 邮箱
    pub email: Option<String>,

    /// 注册来源
    pub data_source_id: Option<i64>,

    /// 登录密码
    pub password: Option<String>,

    /// 确认密码
    pub confirm_password: Option<String>,

    /// 是否认证
    pub is_authed: Option<bool>,

    /// 是否启用
    pub is_enabled: Option<bool>,

    /// 是否测试账号
    pub is_test: Option<bool>,

    /// 详情信息
    pub detail: Option<DetailStoreDTO>,
}

impl UserStoreDTO {
    fn str_to_type_vec(opt: &Option<String>) -> Option<Vec<EditorTypeEnum>> {
        if opt.is_none() {
            return None;
        }

        let str = opt.clone().unwrap();
        let list = EditorTypeEnum::from_comma_str(str.as_str());
        if list.is_empty() { None } else { Some(list) }
    }
}

impl From<UserCreateForm> for UserStoreDTO {
    fn from(model: UserCreateForm) -> Self {
        let mut detail_dto: Option<DetailStoreDTO> = None;
        if model.detail.is_some() {
            detail_dto = Some(model.detail.clone().unwrap().into());
        }
        Self {
            avatar_path: model.avatar_path,
            confirm_password: model.confirm_password,
            data_source_id: model.data_source_id,
            email: model.email,
            gender: model.gender,
            is_authed: model.is_authed,
            is_enabled: model.is_enabled,
            is_test: model.is_test,
            name: model.name,
            nickname: model.nickname,
            no: model.no,
            password: model.password,
            phone: model.phone,
            real_name: model.real_name,
            detail: detail_dto,
            types_list: Self::str_to_type_vec(&model.user_types),
            ..Default::default()
        }
    }
}

impl From<UserUpdateForm> for UserStoreDTO {
    fn from(model: UserUpdateForm) -> Self {
        let mut detail_dto: Option<DetailStoreDTO> = None;
        if model.detail.is_some() {
            detail_dto = Some(model.detail.clone().unwrap().into());
        }
        Self {
            id: model.id,
            avatar_path: model.avatar_path,
            email: model.email,
            gender: model.gender,
            is_authed: model.is_authed,
            is_enabled: model.is_enabled,
            is_test: model.is_test,
            name: model.name,
            nickname: model.nickname,
            no: model.no,
            phone: model.phone,
            real_name: model.real_name,
            detail: detail_dto,
            types_list: Self::str_to_type_vec(&model.user_types),
            ..Default::default()
        }
    }
}
