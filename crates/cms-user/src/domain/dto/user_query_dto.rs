use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

use cms_core::enums::EditorTypeEnum;

use crate::{
    domain::query::UserPaginateQuery,
    enums::{GenderEnum, UserLoadEnum},
};

// ------------------------------------
// 查询用户
// ------------------------------------
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Default)]
pub struct UserQueryDTO {
    /// 页码
    pub page: Option<u64>,

    /// 每页数量
    pub page_size: Option<u64>,

    /// 用户类型
    pub user_types: Option<EditorTypeEnum>,

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
