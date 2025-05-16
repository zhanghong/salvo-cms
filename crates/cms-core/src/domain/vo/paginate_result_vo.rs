use salvo::oapi::ToSchema;
use serde::{Deserialize, Serialize};

// ------------------------------------
// 分页查询 VO
// ------------------------------------
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Default, ToSchema)]
#[salvo(schema(name = "Core::Base::PaginateResultVO"))]
pub struct PaginateResultVO<T: Serialize> {
    /// 当前页码
    #[salvo(schema(required = true, nullable = false, value_type = i64, minimum = 1, default = 1, example = 5))]
    pub current_page: u64,

    /// 每页显示条数
    #[salvo(schema(required = true, nullable = false, value_type = i64, minimum = 1, maximum = 100, example = 50))]
    pub page_size: u64,

    /// 总条数
    #[salvo(schema(required = true, nullable = false, value_type = i64, minimum = 0, example = 50))]
    pub total: u64,

    #[salvo(schema(required = true, nullable = false, value_type = Vec<Object>))]
    pub list: Vec<T>,
}
