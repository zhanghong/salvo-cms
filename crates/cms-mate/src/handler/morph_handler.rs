use salvo::prelude::*;

use cms_core::domain::{AppResult, ResponseSuccess, result_ok};

use crate::domain::query::MorphInstanceQuery;

/// List By Instance
///
/// Get list by instance    
#[endpoint(
    operation_id = "mate_morph_manager_list_by_instance",
    security(["bearer" = ["bearer"]]),
    tags("Mate/Manager/Morph"),
    status_codes(200, 401, 404),
    responses(
        (status_code = 200, body = ResponseSuccess<bool>)
    )
)]
pub async fn manager_list(_depot: &mut Depot, query: MorphInstanceQuery) -> AppResult<bool> {
    print!("=======================================");
    println!("{:#?}", query);
    println!("=====================================");
    result_ok(true)
}
