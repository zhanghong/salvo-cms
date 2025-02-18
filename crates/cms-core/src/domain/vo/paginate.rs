use salvo::oapi::ToSchema;
use serde::{Deserialize, Serialize};

// ------------------------------------
// 分页查询 VO
// ------------------------------------
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Default, ToSchema)]
pub struct PaginateResultVO<T: Serialize> {
    /// 当前页码
    pub current_page: u64,

    /// 每页显示条数
    pub page_size: u64,

    /// 总条数
    pub total: u64,

    pub list: Vec<T>,
}
