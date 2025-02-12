use salvo::prelude::*;

use crate::domain::vo::{result_ok, ApiResult};

#[handler]
pub async fn health() -> ApiResult<String> {
    result_ok("oK".to_string())
}
