use chrono::NaiveDateTime;
use salvo::oapi::ToParameters;
use serde::{Deserialize, Serialize};

use crate::enums::GenderEnum;

// ------------------------------------
// 分页查询用户
// ------------------------------------
#[derive(Deserialize, Serialize, Debug, Clone, Default, ToParameters)]
pub struct UserPaginateQuery {
    /// 页码
    #[salvo(parameter(required = false, nullable = true, value_type =u32, default = 1, minimum = 1))]
    pub page: Option<u64>,

    /// 每页数量
    #[salvo(parameter(required = false, nullable = true, value_type =u32, default = 10, minimum = 1, maximum = 100))]
    pub page_size: Option<u64>,

    /// 关键字
    #[salvo(parameter(required = false, nullable = true))]
    pub keyword: Option<String>,

    /// 手机号
    #[salvo(parameter(required = false, nullable = true))]
    pub phone: Option<String>,

    /// 邮箱
    #[salvo(parameter(required = false, nullable = true))]
    pub email: Option<String>,

    /// 启用状态
    #[salvo(parameter(required = false, nullable = true))]
    pub is_enabled: Option<bool>,

    /// 是否认证
    #[salvo(parameter(required = false, nullable = true))]
    pub is_authed: Option<bool>,

    /// 是否测试账号
    #[salvo(parameter(required = false, nullable = true))]
    pub is_test: Option<bool>,

    /// 性别
    #[salvo(parameter(required = false, nullable = true))]
    pub gender: Option<GenderEnum>,

    /// 登录开始时间
    #[salvo(parameter(required = false, nullable = true))]
    pub login_start_time: Option<NaiveDateTime>,

    /// 登录结束时间
    #[salvo(parameter(required = false, nullable = true))]
    pub login_end_time: Option<NaiveDateTime>,

    /// 创建开始时间
    #[salvo(parameter(required = false, nullable = true))]
    pub created_start_time: Option<NaiveDateTime>,

    /// 创建结束时间
    #[salvo(parameter(required = false, nullable = true))]
    pub created_end_time: Option<NaiveDateTime>,
}
