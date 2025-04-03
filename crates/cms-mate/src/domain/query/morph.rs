use salvo::oapi::ToParameters;
use serde::{Deserialize, Serialize};

use cms_core::utils::deserializer;

// ------------------------------------
// 分页查询 Morph
// ------------------------------------
#[derive(Deserialize, Serialize, Debug, Clone, Default, ToParameters)]
#[salvo(parameters(default_parameter_in = Query))]
#[salvo(schema(name = "Mate模块/Morph/Morph查询条件"))]
pub struct MorphInstanceQuery {
    /// 实例类型
    #[serde(default, deserialize_with = "deserializer::string_to_option_trimmed")]
    #[salvo(parameter(required = true, nullable = true))]
    pub instance_type: Option<String>,

    /// 实例ID
    #[serde(default, deserialize_with = "deserializer::string_to_option_i64")]
    #[salvo(parameter(required = true, nullable = true))]
    pub instance_id: Option<i64>,

    /// 类型列表
    #[serde(
        default,
        deserialize_with = "deserializer::string_to_option_string_vec"
    )]
    #[salvo(parameter(required = false, nullable = true))]
    pub kind_names: Option<Vec<String>>,
}
