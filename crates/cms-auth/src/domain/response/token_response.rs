use std::fmt::Debug;

use salvo::oapi::ToSchema;
use serde::Serialize;

use crate::domain::vo::{TokenCreateVO, TokenUpdateVO};

/// Token Create Response
#[derive(Debug, Serialize, ToSchema)]
#[salvo(schema(name = "Auth::Token::TokenCreateResponse"))]
pub struct TokenCreateResponse {
    /// 状态码
    #[salvo(schema(required = true, minimum = 200, maximum = 299, example = 200))]
    code: u32,

    /// 返回数据
    #[salvo(schema(required = true, max_length = 200, example = "OK"))]
    data: TokenCreateVO,
}

/// Token Update Response
#[derive(Debug, Serialize, ToSchema)]
#[salvo(schema(name = "Auth::Token::TokenUpdateResponse"))]
pub struct TokenUpdateResponse {
    /// 状态码
    #[salvo(schema(required = true, minimum = 200, maximum = 299, example = 200))]
    code: u32,

    /// 返回数据
    #[salvo(schema(required = true, max_length = 200, example = "OK"))]
    data: TokenUpdateVO,
}
