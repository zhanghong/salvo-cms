use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

use crate::{
    domain::{
        form::{UserCreateForm, UserUpdateForm, UserUpdatePasswordForm},
        query::UserPaginateQuery,
    },
    enums::{GenderEnum, UserLoadEnum, UserTypeEnum},
};

use super::DetailStoreDTO;

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

    /// 详情信息
    pub detail: Option<DetailStoreDTO>,
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
            realname: model.realname,
            detail: detail_dto,
            types_list: Self::str_to_type_vec(&model.user_type),
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
            realname: model.realname,
            detail: detail_dto,
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
    pub id: i64,

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
            id: 0,
            current_password: model.current_password,
            confirm_password: model.confirm_password.unwrap(),
            new_password: model.new_password.unwrap(),
        }
    }
}

// ------------------------------------
// 查看用户
// ------------------------------------
// Service 层查看用户使用的结构体
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Default)]
pub struct UserViewDTO {
    /// 主键
    pub id: i64,

    /// 当前密码
    pub user_type: Option<UserTypeEnum>,

    /// 确认密码
    pub enabled: Option<bool>,
}

// ------------------------------------
// 查询用户
// ------------------------------------
// Service 层查询用户使用的结构体
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Default)]
pub struct UserQueryDTO {
    /// 页码
    pub page: Option<u64>,

    /// 每页数量
    pub page_size: Option<u64>,

    /// 用户类型
    pub user_type: Option<UserTypeEnum>,

    /// 关键字
    pub keyword: Option<String>,

    /// 手机号
    pub phone: Option<String>,

    /// 邮箱
    pub email: Option<String>,

    /// 启用状态
    pub is_enabled: Option<bool>,

    /// 是否认证
    pub is_authed: Option<bool>,

    /// 是否测试账号
    pub is_test: Option<bool>,

    /// 性别
    pub gender: Option<GenderEnum>,

    /// 登录开始时间
    pub login_start_time: Option<NaiveDateTime>,

    /// 登录结束时间
    pub login_end_time: Option<NaiveDateTime>,

    /// 创建开始时间
    pub created_start_time: Option<NaiveDateTime>,

    /// 创建结束时间
    pub created_end_time: Option<NaiveDateTime>,

    /// 加载关联数据
    pub load_models: Option<Vec<UserLoadEnum>>,
}

impl From<UserPaginateQuery> for UserQueryDTO {
    fn from(model: UserPaginateQuery) -> Self {
        Self {
            page: model.page,
            page_size: model.page_size,
            keyword: model.keyword,
            phone: model.phone,
            email: model.email,
            is_enabled: model.is_enabled,
            is_authed: model.is_authed,
            is_test: model.is_test,
            gender: model.gender,
            login_start_time: model.login_start_time,
            login_end_time: model.login_end_time,
            created_start_time: model.created_start_time,
            created_end_time: model.created_end_time,
            ..Default::default()
        }
    }
}
