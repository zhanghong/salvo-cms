use std::collections::HashMap;
use std::fmt::Debug;

use salvo::oapi::ToSchema;
use serde::Serialize;
use uuid::Uuid;

// Fail Response
#[derive(Debug, Serialize, ToSchema)]
#[salvo(schema(name = "Core::Base::Response::BaseErrorResponse"))]
pub struct BaseErrorResponse {
    /// Status Code
    #[salvo(schema(required = true, nullable = false, value_type = u32, minimum=300, maximum = 600, example = 500))]
    code: u32,

    /// Error Message
    #[salvo(schema(
        required = true,
        nullable = false,
        max_length = 200,
        example = "Internal Server Error"
    ))]
    message: String,

    /// Response Data
    #[serde(skip_serializing_if = "Option::is_none")]
    #[salvo(
    schema(
        required = false, 
        nullable = true, 
        value_type = HashMap<String, String>, 
        example = json!({
            "name": "name is required",
            "title": "title must be between 2 and 10 characters long"
        })
      )
    )]
    data: Option<HashMap<String, String>>,
}

/// Return string response
#[derive(Debug, Serialize, ToSchema)]
#[salvo(schema(name = "Core::Base::Response::BaseStringResponse"))]
pub struct BaseStringResponse {
    /// Status Code
    #[salvo(schema(required = true, nullable = false, value_type = i32, minimum = 200, maximum = 299, example = 200))]
    code: u32,

    /// Response Data
    #[salvo(schema(required = true, nullable = false, max_length = 200, example = "OK"))]
    data: String,
}

/// return boolean response
#[derive(Debug, Serialize, ToSchema)]
#[salvo(schema(name = "Core::Base::Response::BaseBooleanResponse"))]
pub struct BaseBooleanResponse {
    /// Status Code
    #[salvo(schema(required = true, nullable = false, value_type = i32, minimum = 200, maximum = 299, example = 200))]
    code: u32,

    /// Response Data
    #[salvo(schema(required = true, nullable = false, example = true))]
    data: bool,
}

/// return uuid response
#[derive(Debug, Serialize, ToSchema)]
#[salvo(schema(name = "Core::Base::Response::BaseUuidResponse"))]
pub struct BaseUuidResponse {
    /// Status Code
    #[salvo(schema(required = true, nullable = false, value_type = i32, minimum = 200, maximum = 299, example = 200))]
    code: u32,

    /// Response Data
    #[salvo(schema(required = true, nullable = false, example = true))]
    data: Uuid,
}

/// return u64 response
#[derive(Debug, Serialize, ToSchema)]
#[salvo(schema(name = "Core::Base::Response::BaseBigIntResponse"))]
pub struct BaseBigIntResponse {
    /// Status Code
    #[salvo(schema(required = true, nullable = false, value_type = i32, minimum = 200, maximum = 299, example = 200))]
    code: u32,

    /// Response Data
    #[salvo(schema(required = true, nullable = false, example = true))]
    data: u64,
}

/// return u32 response
#[derive(Debug, Serialize, ToSchema)]
#[salvo(schema(name = "Core::Base::Response::BaseMiddleIntResponse"))]
pub struct BaseMiddleIntResponse {
    /// Error Message
    #[salvo(schema(required = true, nullable = false, value_type = i32, minimum = 200, maximum = 299, example = 200))]
    code: u32,

    /// Response Data
    #[salvo(schema(required = true, nullable = false, example = true))]
    data: u32,
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;
    use std::collections::HashMap;

    /// 工具函数：验证序列化结果
    fn assert_serialize<T: Serialize>(value: &T, expected: serde_json::Value) {
        let json = serde_json::to_value(value).unwrap();
        assert_eq!(json, expected);
    }

    #[test]
    fn test_base_error_response() {
        // 测试基本构造和序列化
        let response = BaseErrorResponse {
            code: 500,
            message: "Internal Server Error".to_string(),
            data: None,
        };
        assert_serialize(
            &response,
            json!({
                "code": 500,
                "message": "Internal Server Error"
            })
        );

        // 测试带data字段的情况
        let mut data = HashMap::new();
        data.insert("name".to_string(), "name is required".to_string());
        data.insert("title".to_string(), "title must be between 2 and 10 characters long".to_string());

        let response = BaseErrorResponse {
            code: 400,
            message: "Validation Failed".to_string(),
            data: Some(data),
        };
        assert_serialize(
            &response,
            json!({
                "code": 400,
                "message": "Validation Failed",
                "data": {
                    "name": "name is required",
                    "title": "title must be between 2 and 10 characters long"
                }
            })
        );
        
        // 测试边界情况
        assert!(response.code >= 300 && response.code <= 600);
    }

    #[test]
    fn test_base_string_response() {
        let response = BaseStringResponse {
            code: 200,
            data: "OK".to_string(),
        };
        assert_serialize(
            &response,
            json!({
                "code": 200,
                "data": "OK"
            })
        );
        
        // 测试边界情况
        assert!(response.code >= 200 && response.code <= 299);
    }

    #[test]
    fn test_base_boolean_response() {
        let response = BaseBooleanResponse {
            code: 200,
            data: true,
        };
        assert_serialize(
            &response,
            json!({
                "code": 200,
                "data": true
            })
        );
        
        // 测试边界情况
        assert!(response.code >= 200 && response.code <= 299);
    }

    #[test]
    fn test_base_uuid_response() {
        let uuid = Uuid::new_v4();
        let response = BaseUuidResponse {
            code: 200,
            data: uuid,
        };
        assert_serialize(
            &response,
            json!({
                "code": 200,
                "data": uuid.to_string()
            })
        );
        
        // 测试边界情况
        assert!(response.code >= 200 && response.code <= 299);
    }

    #[test]
    fn test_base_big_int_response() {
        let response = BaseBigIntResponse {
            code: 200,
            data: 12345678,
        };
        assert_serialize(
            &response,
            json!({
                "code": 200,
                "data": 12345678
            })
        );
        
        // 测试边界情况
        assert!(response.code >= 200 && response.code <= 299);
    }

    #[test]
    fn test_base_middle_int_response() {
        let response = BaseMiddleIntResponse {
            code: 200,
            data: 25648,
        };
        assert_serialize(
            &response,
            json!({
                "code": 200,
                "data": 25648
            })
        );
        
        // 测试边界情况
        assert!(response.code >= 200 && response.code <= 299);
    }

    #[test]
    #[should_panic(expected = "assertion failed")]
    fn test_invalid_status_code_for_success_response() {
        // 测试无效的状态码（应该失败）
        let response = BaseStringResponse {
            code: 300,
            data: "Invalid".to_string(),
        };
        assert!(response.code >= 200 && response.code <= 299);
    }

    #[test]
    #[should_panic(expected = "assertion failed")]
    fn test_invalid_status_code_for_error_response() {
        // 测试无效的错误状态码（应该失败）
        let response = BaseErrorResponse {
            code: 200,
            message: "Invalid".to_string(),
            data: None,
        };
        assert!(response.code >= 300 && response.code <= 600);
    }
}