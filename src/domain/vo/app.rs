use std::fmt::Debug;

use salvo::prelude::*;
use serde::Serialize;

// 统一响应结构
#[derive(Debug, Serialize)]
pub struct ApiResponse<T: Serialize> {
    code: u32,
    message: Option<String>,
    data: Option<T>,
}

impl<T: Serialize> ApiResponse<T> {
    // 成功响应
    pub fn success(data: T) -> Self {
        Self {
            code: 200,
            message: None,
            data: Some(data),
        }
    }

    // 错误响应
    pub fn error(code: u32, message: &str) -> Self {
        Self {
            code,
            message: Some(message.to_string()),
            data: None,
        }
    }
}

#[async_trait]
impl<T> Writer for ApiResponse<T>
where
    T: Serialize + Send + Sync + Debug + 'static,
{
    async fn write(mut self, _req: &mut Request, depot: &mut Depot, res: &mut Response) {
        let json_string = serde_json::to_string(&self).unwrap_or_default();
        depot.insert("res_v", json_string);
        res.render(Json(&self));
    }
}
