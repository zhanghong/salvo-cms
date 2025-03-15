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
    #[salvo(parameter(required = true, nullable = true, value_type = String))]
    pub instance_type: Option<String>,

    /// 实例ID
    #[serde(default, deserialize_with = "deserializer::string_to_option_i64")]
    #[salvo(parameter(required = true, nullable = true, value_type = i64))]
    pub instance_id: Option<i64>,

    /// 类型列表
    #[serde(default)]
    #[salvo(parameter(required = false, nullable = true, value_type = Vec<String>))]
    pub kind_names: Option<Vec<String>>,
}
