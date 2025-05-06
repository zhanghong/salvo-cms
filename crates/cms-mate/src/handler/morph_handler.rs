use salvo::prelude::*;

use cms_core::domain::{AppResult, result_ok};

use crate::domain::query::MorphInstanceQuery;

/// 关联Item列表
///
/// 管理端查询 Instance 关联Item列表
#[endpoint(parameters(MorphInstanceQuery), tags("Mate模块/管理端/Morph管理"))]
pub async fn manager_list(_depot: &mut Depot, query: MorphInstanceQuery) -> AppResult<bool> {
    print!("=======================================");
    println!("{:#?}", query);
    println!("=====================================");
    result_ok(true)
}
