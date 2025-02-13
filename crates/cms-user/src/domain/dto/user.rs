use serde::{Deserialize, Serialize};

use cms_core::enums::PlatformEnum;

use crate::domain::form::{UserCreateForm, UserUpdateForm, UserUpdatePasswordForm};

// ------------------------------------
// 创建/更新用户
// ------------------------------------
// Service 层创建/更新用户使用的结构体
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Default)]
pub struct UserStoreDTO {
    /// 主键
    pub id: Option<i64>,

    /// 操作来源
    pub platform_enum: Option<PlatformEnum>,

    /// 头像URL
    pub avatar_path: Option<String>,

    /// 确认密码
    pub confirm_password: Option<String>,

    /// 注册来源
    pub data_source_id: Option<i64>,

    /// 邮箱
    pub email: Option<String>,

    /// 性别
    pub gender: Option<i16>,

    /// 是否认证
    pub is_authed: Option<bool>,

    /// 是否启用
    pub is_enabled: Option<bool>,

    /// 是否测试账号
    pub is_test: Option<bool>,

    /// 登录名
    pub name: Option<String>,

    /// 昵称
    pub nickname: Option<String>,

    /// NO
    pub no: Option<String>,

    /// 登录密码
    pub password: Option<String>,

    /// 手机号码
    pub phone: Option<String>,

    /// 真实姓名
    pub realname: Option<String>,

    /// 角色类型
    pub user_type: Option<String>,
}

impl From<UserCreateForm> for UserStoreDTO {
    fn from(dto: UserCreateForm) -> Self {
        Self {
            platform_enum: dto.platform_enum,
            avatar_path: dto.avatar_path,
            confirm_password: dto.confirm_password,
            data_source_id: dto.data_source_id,
            email: dto.email,
            gender: dto.gender,
            is_authed: dto.is_authed,
            is_enabled: dto.is_enabled,
            is_test: dto.is_test,
            name: dto.name,
            nickname: dto.nickname,
            no: dto.no,
            password: dto.password,
            phone: dto.phone,
            realname: dto.realname,
            user_type: dto.user_type,
            ..Default::default()
        }
    }
}

impl From<UserUpdateForm> for UserStoreDTO {
    fn from(dto: UserUpdateForm) -> Self {
        Self {
            id: dto.id,
            platform_enum: dto.platform_enum,
            avatar_path: dto.avatar_path,
            email: dto.email,
            gender: dto.gender,
            is_authed: dto.is_authed,
            is_enabled: dto.is_enabled,
            is_test: dto.is_test,
            name: dto.name,
            nickname: dto.nickname,
            no: dto.no,
            phone: dto.phone,
            realname: dto.realname,
            user_type: dto.user_type,
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

    /// 操作来源
    pub platform_enum: Option<PlatformEnum>,

    /// 当前密码
    pub current_password: Option<String>,

    /// 确认密码
    pub confirm_password: String,

    /// 新密码
    pub new_password: String,
}

impl From<UserUpdatePasswordForm> for UserUpdatePasswordDTO {
    fn from(dto: UserUpdatePasswordForm) -> Self {
        Self {
            id: dto.id.unwrap(),
            platform_enum: dto.platform_enum,
            current_password: dto.current_password,
            confirm_password: dto.confirm_password.unwrap(),
            new_password: dto.new_password.unwrap(),
        }
    }
}
