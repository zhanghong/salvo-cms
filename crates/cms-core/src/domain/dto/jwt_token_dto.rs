use serde::{Deserialize, Serialize};

/// Jwt Token DTO
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Default)]
pub struct JwtTokenDTO {
    pub token_type: String,
    pub token_value: String,
    pub expired_time: i64,
}

#[cfg(test)]
mod tests {
    use super::*;

    // T01: 测试默认构造函数是否初始化字段
    #[test]
    fn test_default_constructor() {
        let dto = JwtTokenDTO::default();
        assert_eq!(dto.token_type, "");
        assert_eq!(dto.token_value, "");
        assert_eq!(dto.expired_time, 0);
    }

    // T02: 测试 Clone trait 是否有效
    #[test]
    fn test_clone_trait() {
        let dto = JwtTokenDTO {
            token_type: "Bearer".to_string(),
            token_value: "abc123xyz".to_string(),
            expired_time: 1717029200,
        };
        let cloned = dto.clone();
        assert_eq!(dto, cloned);
    }

    // T03: 测试 Serialize/Deserialize 是否保持一致性
    #[test]
    fn test_serde_json_round_trip() {
        let original = JwtTokenDTO {
            token_type: "Bearer".to_string(),
            token_value: "abc123xyz".to_string(),
            expired_time: 1717029200,
        };

        let serialized = serde_json::to_string(&original).unwrap();
        let deserialized: JwtTokenDTO = serde_json::from_str(&serialized).unwrap();

        assert_eq!(original, deserialized);
    }

    // T04: 测试手动设置字段是否生效
    #[test]
    fn test_manual_field_assignment() {
        let mut dto = JwtTokenDTO::default();
        dto.token_type = "JWT".to_string();
        dto.token_value = "xyz789".to_string();
        dto.expired_time = 1717030000;

        assert_eq!(dto.token_type, "JWT");
        assert_eq!(dto.token_value, "xyz789");
        assert_eq!(dto.expired_time, 1717030000);
    }

    // T05: 测试 PartialEq trait 是否正确
    #[test]
    fn test_partial_eq_trait() {
        let a = JwtTokenDTO {
            token_type: "Bearer".to_string(),
            token_value: "abc123".to_string(),
            expired_time: 1717029200,
        };

        let b = a.clone();
        let c = JwtTokenDTO {
            token_type: "Bearer".to_string(),
            token_value: "wrong_token".to_string(),
            expired_time: 1717029200,
        };

        assert_eq!(a, b); // 相同应为 true
        assert_ne!(a, c); // 不同应为 false
    }
}
