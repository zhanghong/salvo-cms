use salvo::oapi::ToParameters;
use serde::{Deserialize, Serialize};

use cms_core::utils::deserializer_utils;

/// Morph分页查询条件
#[derive(Deserialize, Serialize, Debug, Clone, Default, ToParameters)]
#[salvo(parameters(default_parameter_in = Query))]
#[salvo(schema(name = "Mate/Morph/MorphInstanceQuery"))]
pub struct MorphInstanceQuery {
    /// 实例类型
    #[serde(
        default,
        deserialize_with = "deserializer_utils::string_to_option_trimmed"
    )]
    #[salvo(parameter(required = true, nullable = false, value_type = String, example = "admin"))]
    pub instance_type: Option<String>,

    /// 实例ID
    #[serde(default, deserialize_with = "deserializer_utils::string_to_option_i64")]
    #[salvo(parameter(required = true, nullable = false, value_type = i64, example = 23))]
    pub instance_id: Option<i64>,

    /// 类型列表
    #[serde(
        default,
        deserialize_with = "deserializer_utils::string_to_option_string_vec"
    )]
    #[salvo(schema(required = true, nullable = false, value_type = Vec<String>, example = json!(["admin", "user"])))]
    pub kind_names: Option<Vec<String>>,
}
