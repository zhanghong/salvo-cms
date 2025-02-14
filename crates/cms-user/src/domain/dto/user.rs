use serde::{Deserialize, Serialize};

use crate::{
    domain::form::{UserCreateForm, UserUpdateForm, UserUpdatePasswordForm},
    enums::{GenderEnum, UserTypeEnum},
};

// ------------------------------------
// 创建/更新用户
// ------------------------------------
// Service 层创建/更新用户使用的结构体
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Default)]
pub struct UserStoreDTO {
    /// 主键
    pub id: Option<i64>,

    /// NO
    pub no: Option<String>,

    /// 登录名
    pub name: Option<String>,

    /// 真实姓名
    pub realname: Option<String>,

    /// 昵称
    pub nickname: Option<String>,

    /// 角色类型
    pub types_list: Option<Vec<UserTypeEnum>>,

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
}

impl UserStoreDTO {
    fn str_to_type_vec(opt: &Option<String>) -> Option<Vec<UserTypeEnum>> {
        if opt.is_none() {
            return None;
        }

        let str = opt.clone().unwrap();
        let list = UserTypeEnum::from_comma_str(str.as_str());
        if list.is_empty() {
            None
        } else {
            Some(list)
        }
    }
}

impl From<UserCreateForm> for UserStoreDTO {
    fn from(model: UserCreateForm) -> Self {
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
            realname: model.realname,
            types_list: Self::str_to_type_vec(&model.user_type),
            ..Default::default()
        }
    }
}

impl From<UserUpdateForm> for UserStoreDTO {
    fn from(model: UserUpdateForm) -> Self {
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
            realname: model.realname,
            types_list: Self::str_to_type_vec(&model.user_type),
            ..Default::default()
        }
    }
}

// ------------------------------------
// 创建/更新用户
// ------------------------------------
// Service 层创建/更新用户使用的结构体
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Default)]
pub struct UserUpdatePasswordDTO {
    /// 主键
    pub id: Option<i64>,

    /// 当前密码
    pub current_password: Option<String>,

    /// 确认密码
    pub confirm_password: String,

    /// 新密码
    pub new_password: String,
}

impl From<UserUpdatePasswordForm> for UserUpdatePasswordDTO {
    fn from(model: UserUpdatePasswordForm) -> Self {
        Self {
            id: model.id,
            current_password: model.current_password,
            confirm_password: model.confirm_password.unwrap(),
            new_password: model.new_password.unwrap(),
        }
    }
}
