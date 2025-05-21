use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Default)]
pub struct JwtClaimsDTO {
    pub uuid: String,
    pub user_id: String,
    pub user_type: String,
    pub token_type: String,
    pub exp: i64,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_values() {
        let claims = JwtClaimsDTO::default();
        assert_eq!(claims.uuid, "");
        assert_eq!(claims.user_id, "");
        assert_eq!(claims.user_type, "");
        assert_eq!(claims.token_type, "");
        assert_eq!(claims.exp, 0);
    }

    #[test]
    fn test_clone() {
        let claims = JwtClaimsDTO {
            uuid: "uuid123".to_string(),
            user_id: "user123".to_string(),
            user_type: "admin".to_string(),
            token_type: "bearer".to_string(),
            exp: 1717029203,
        };
        let cloned = claims.clone();
        assert_eq!(claims, cloned);
    }

    #[test]
    fn test_serialize() {
        let claims = JwtClaimsDTO {
            uuid: "uuid123".to_string(),
            user_id: "user123".to_string(),
            user_type: "admin".to_string(),
            token_type: "bearer".to_string(),
            exp: 1717029203,
        };
        let json_str = serde_json::to_string(&claims).unwrap();
        let expected_json = r#"{"uuid":"uuid123","user_id":"user123","user_type":"admin","token_type":"bearer","exp":1717029203}"#;
        assert_eq!(json_str, expected_json);
    }

    #[test]
    fn test_deserialize() {
        let json_str = r#"{"uuid":"uuid123","user_id":"user123","user_type":"admin","token_type":"bearer","exp":1717029203}"#;
        let claims: JwtClaimsDTO = serde_json::from_str(json_str).unwrap();
        assert_eq!(claims.uuid, "uuid123");
        assert_eq!(claims.user_id, "user123");
        assert_eq!(claims.user_type, "admin");
        assert_eq!(claims.token_type, "bearer");
        assert_eq!(claims.exp, 1717029203);
    }

    #[test]
    fn test_partial_eq() {
        let claims1 = JwtClaimsDTO {
            uuid: "uuid123".to_string(),
            user_id: "user123".to_string(),
            user_type: "admin".to_string(),
            token_type: "bearer".to_string(),
            exp: 1717029203,
        };

        let claims2 = claims1.clone();
        let claims3 = JwtClaimsDTO {
            uuid: "uuid456".to_string(),
            ..claims1.clone()
        };

        assert!(claims1 == claims2); // Same values
        assert!(claims1 != claims3); // Different uuid
    }
}
