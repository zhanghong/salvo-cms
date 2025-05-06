use chrono::NaiveDateTime;
use salvo::oapi::ToParameters;
use serde::{Deserialize, Serialize};

use cms_core::utils::{deserializer_utils, parameter_utils};

/// Kind 查询条件
#[derive(Deserialize, Serialize, Debug, Clone, Default, ToParameters)]
#[salvo(parameters(default_parameter_in = Query))]
#[salvo(schema(name = "Mate模块/Kind/Kind查询条件"))]
pub struct KindPaginateQuery {
    /// 页码
    #[serde(
        default = "parameter_utils::page_no_default",
        deserialize_with = "deserializer_utils::string_to_param_page_no"
    )]
    #[salvo(parameter(required = false, nullable = false, default = 1, minimum = 1))]
    pub page: u64,

    /// 每页数量
    #[serde(
        default = "parameter_utils::page_size_default",
        deserialize_with = "deserializer_utils::string_to_param_page_size"
    )]
    #[salvo(parameter(
        required = false,
        nullable = false,
        default = 10,
        minimum = 1,
        maximum = 100
    ))]
    pub page_size: u64,

    /// App ID
    #[serde(default, deserialize_with = "deserializer_utils::string_to_option_i64")]
    #[salvo(parameter(required = false, nullable = false, value_type = i64, minimum = 0, example = 1))]
    pub app_id: Option<i64>,

    /// 关键字
    #[serde(
        default,
        deserialize_with = "deserializer_utils::string_to_option_trimmed"
    )]
    #[salvo(parameter(required = false, nullable = false, value_type = String, example = "admin"))]
    pub keyword: Option<String>,

    /// 标题
    #[serde(
        default,
        deserialize_with = "deserializer_utils::string_to_option_trimmed"
    )]
    #[salvo(parameter(required = false, nullable = false, value_type = String, example = "商品"))]
    pub title: Option<String>,

    /// 启用状态
    #[serde(
        default,
        deserialize_with = "deserializer_utils::string_to_option_bool"
    )]
    #[salvo(parameter(required = false, nullable = false, value_type = bool, example = true))]
    pub is_enabled: Option<bool>,

    /// 创建开始时间
    #[serde(
        default,
        deserialize_with = "deserializer_utils::string_to_option_naive_datetime"
    )]
    #[salvo(parameter(required = false, nullable = false, value_type = String, format = "yyyy-MM-dd", example = "2023-01-01"))]
    pub created_start_time: Option<NaiveDateTime>,

    /// 创建结束时间
    #[serde(
        default,
        deserialize_with = "deserializer_utils::string_to_option_naive_datetime"
    )]
    #[salvo(parameter(required = false, nullable = false, value_type = String, format = "yyyy-MM-dd", example = "2023-01-01"))]
    pub created_end_time: Option<NaiveDateTime>,
}
