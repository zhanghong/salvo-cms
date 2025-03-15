use chrono::NaiveDateTime;
use salvo::oapi::{ToParameters, ToSchema};
use serde::{Deserialize, Serialize};

use cms_core::utils::{deserializer, parameter};

// ------------------------------------
// 分页查询 App
// ------------------------------------
#[derive(Deserialize, Serialize, Debug, Clone, Default, ToParameters, ToSchema)]
#[salvo(parameters(default_parameter_in = Query))]
#[salvo(schema(name = "Mate模块/App/App查询条件"))]
pub struct AppPaginateQuery {
    /// 页码
    #[serde(
        default = "parameter::page_no_default",
        deserialize_with = "deserializer::string_to_param_page_no"
    )]
    #[salvo(parameter(required = false, nullable = true, value_type =u32, default = 1, minimum = 1))]
    pub page: u64,

    /// 每页数量
    #[serde(
        default = "parameter::page_size_default",
        deserialize_with = "deserializer::string_to_param_page_size"
    )]
    #[salvo(parameter(required = false, nullable = true, value_type =u32, default = 10, minimum = 1, maximum = 100))]
    pub page_size: u64,

    /// 关键字
    #[serde(default, deserialize_with = "deserializer::string_to_option_trimmed")]
    #[salvo(parameter(required = false, nullable = true))]
    pub keyword: Option<String>,

    /// 标题
    #[serde(default, deserialize_with = "deserializer::string_to_option_trimmed")]
    #[salvo(parameter(required = false, nullable = true))]
    pub title: Option<String>,

    /// 启用状态
    #[serde(default, deserialize_with = "deserializer::string_to_option_bool")]
    #[salvo(parameter(required = false, nullable = true))]
    pub is_enabled: Option<bool>,

    /// 创建开始时间
    #[serde(
        default,
        deserialize_with = "deserializer::string_to_option_naive_datetime"
    )]
    #[salvo(parameter(required = false, nullable = true, value_type = NaiveDateTime))]
    pub created_start_time: Option<NaiveDateTime>,

    /// 创建结束时间
    #[serde(
        default,
        deserialize_with = "deserializer::string_to_option_naive_datetime"
    )]
    #[salvo(parameter(required = false, nullable = true, value_type = NaiveDateTime))]
    pub created_end_time: Option<NaiveDateTime>,
}
