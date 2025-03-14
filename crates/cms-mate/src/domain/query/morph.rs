use salvo::oapi::ToParameters;
use serde::{Deserialize, Serialize};

use cms_core::utils::deserializer;

// ------------------------------------
// 分页查询用户
// ------------------------------------
#[derive(Deserialize, Serialize, Debug, Clone, Default, ToParameters)]
pub struct MorphInstanceQuery {
    /// 实例类型
    #[serde(deserialize_with = "deserializer::string_to_option_trimmed")]
    #[salvo(parameter(required = true, nullable = true, value_type = String))]
    pub instance_type: Option<String>,

    /// 实例ID
    #[serde(deserialize_with = "deserializer::string_to_option_i64")]
    #[salvo(parameter(required = true, nullable = true, value_type = i64))]
    pub instance_id: Option<i64>,

    /// 类型列表
    #[salvo(parameter(required = false, nullable = true, value_type = Vec<String>))]
    pub kind_names: Option<Vec<String>>,
}
