use salvo::oapi::ToParameters;
use serde::{Deserialize, Serialize};

// ------------------------------------
// 分页查询用户
// ------------------------------------
#[derive(Deserialize, Serialize, Debug, Clone, Default, ToParameters)]
pub struct MorphInstanceQuery {
    /// 模块ID
    #[salvo(parameter(required = true, nullable = true, value_type = String))]
    pub instance_type: Option<String>,

    /// 类型ID
    #[salvo(parameter(required = true, nullable = true, value_type = i64))]
    pub instance_id: Option<i64>,

    /// 父级ID
    #[salvo(parameter(required = false, nullable = true, value_type = Vec<i64>))]
    pub kind_ids: Option<Vec<i64>>,
}
