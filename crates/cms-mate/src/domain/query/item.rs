use chrono::NaiveDateTime;
use salvo::oapi::ToParameters;
use serde::{Deserialize, Serialize};

use cms_core::utils::deserializer;

// ------------------------------------
// 分页查询用户
// ------------------------------------
#[derive(Deserialize, Serialize, Debug, Clone, Default, ToParameters)]
pub struct ItemPaginateQuery {
    /// 页码
    #[serde(deserialize_with = "deserializer::string_to_option_u64")]
    #[salvo(parameter(required = false, nullable = true, value_type =u32, default = 1, minimum = 1))]
    pub page: Option<u64>,

    /// 每页数量
    #[serde(deserialize_with = "deserializer::string_to_option_u64")]
    #[salvo(parameter(required = false, nullable = true, value_type =u32, default = 10, minimum = 1, maximum = 100))]
    pub page_size: Option<u64>,

    /// App ID
    #[serde(deserialize_with = "deserializer::string_to_option_i64")]
    #[salvo(parameter(required = false, nullable = true, value_type =i64, default = 10, minimum = 1, maximum = 100))]
    pub app_id: Option<i64>,

    /// 类型ID
    #[serde(deserialize_with = "deserializer::string_to_option_i64")]
    #[salvo(parameter(required = false, nullable = true, value_type =i64, default = 10, minimum = 1, maximum = 100))]
    pub kind_id: Option<i64>,

    /// 父级ID
    #[serde(deserialize_with = "deserializer::string_to_option_i64")]
    #[salvo(parameter(required = false, nullable = true, value_type =i64, default = 10, minimum = 1, maximum = 100))]
    pub parent_id: Option<i64>,

    /// 关键字
    #[serde(deserialize_with = "deserializer::string_to_option_trimmed")]
    #[salvo(parameter(required = false, nullable = true))]
    pub keyword: Option<String>,

    /// 手机号
    #[serde(deserialize_with = "deserializer::string_to_option_trimmed")]
    #[salvo(parameter(required = false, nullable = true))]
    pub title: Option<String>,

    /// 启用状态
    #[serde(deserialize_with = "deserializer::string_to_option_bool")]
    #[salvo(parameter(required = false, nullable = true))]
    pub is_enabled: Option<bool>,

    /// 创建开始时间
    #[salvo(parameter(required = false, nullable = true))]
    pub created_start_time: Option<NaiveDateTime>,

    /// 创建结束时间
    #[salvo(parameter(required = false, nullable = true))]
    pub created_end_time: Option<NaiveDateTime>,
}
