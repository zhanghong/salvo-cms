use salvo::prelude::*;

#[handler]
pub async fn health() -> &'static str {
    "Ok"
}
