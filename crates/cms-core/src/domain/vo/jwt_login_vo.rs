use salvo::oapi::ToSchema;
use serde::{Deserialize, Serialize};

/// JWT Login VO
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Default, ToSchema)]
#[salvo(schema(name = "Core::Auth::JwtLoginVO"))]
pub struct JwtLoginVO {
    /// Access Token
    #[salvo(schema(required = true, nullable = false))]
    pub access_token: String,

    /// Access Token 过期时间
    #[salvo(schema(required = true, nullable = false, example = 3600))]
    pub access_expired: i64,

    /// Refresh Token
    #[salvo(schema(required = true, nullable = false))]
    pub refresh_token: String,

    /// Refresh Token 过期时间
    #[salvo(schema(required = true, nullable = false, example = 86400))]
    pub refresh_expired: i64,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_jwt_login_vo_creation() {
        let vo = JwtLoginVO {
            access_token: "access_abc123".to_string(),
            access_expired: 3600,
            refresh_token: "refresh_xyz789".to_string(),
            refresh_expired: 86400,
        };

        assert_eq!(vo.access_token, "access_abc123");
        assert_eq!(vo.access_expired, 3600);
        assert_eq!(vo.refresh_token, "refresh_xyz789");
        assert_eq!(vo.refresh_expired, 86400);
    }

    // TC02: 默认初始化结构体
    #[test]
    fn test_jwt_login_vo_default() {
        let vo = JwtLoginVO::default();

        assert_eq!(vo.access_token, "");
        assert_eq!(vo.access_expired, 0);
        assert_eq!(vo.refresh_token, "");
        assert_eq!(vo.refresh_expired, 0);
    }

    // TC03: 序列化结构体为 JSON
    #[test]
    fn test_jwt_login_vo_serialize() {
        let vo = JwtLoginVO {
            access_token: "access_abc123".to_string(),
            access_expired: 3600,
            refresh_token: "refresh_xyz789".to_string(),
            refresh_expired: 86400,
        };

        let json = serde_json::to_string(&vo).unwrap();
        let expected_json = r#"{"access_token":"access_abc123","access_expired":3600,"refresh_token":"refresh_xyz789","refresh_expired":86400}"#;

        assert_eq!(json, expected_json);
    }

    // TC04: 反序列化 JSON 到结构体
    #[test]
    fn test_jwt_login_vo_deserialize() {
        let json = r#"{"access_token":"access_abc123","access_expired":3600,"refresh_token":"refresh_xyz789","refresh_expired":86400}"#;
        let vo: JwtLoginVO = serde_json::from_str(json).unwrap();

        assert_eq!(vo.access_token, "access_abc123");
        assert_eq!(vo.access_expired, 3600);
        assert_eq!(vo.refresh_token, "refresh_xyz789");
        assert_eq!(vo.refresh_expired, 86400);
    }

    #[test]
    fn test_jwt_login_vo_partial_eq_same() {
        let vo1 = JwtLoginVO {
            access_token: "token1".to_string(),
            access_expired: 3600,
            refresh_token: "refresh1".to_string(),
            refresh_expired: 86400,
        };
        let vo2 = vo1.clone();

        assert_eq!(vo1, vo2);
    }

    #[test]
    fn test_jwt_login_vo_partial_eq_different() {
        let vo1 = JwtLoginVO {
            access_token: "token1".to_string(),
            access_expired: 3600,
            refresh_token: "refresh1".to_string(),
            refresh_expired: 86400,
        };
        let vo2 = JwtLoginVO {
            access_token: "token2".to_string(),
            access_expired: 7200,
            refresh_token: "refresh2".to_string(),
            refresh_expired: 172800,
        };

        assert_ne!(vo1, vo2);
    }

    #[test]
    fn test_jwt_login_vo_clone() {
        let vo = JwtLoginVO {
            access_token: "clone_token".to_string(),
            access_expired: 3600,
            refresh_token: "clone_refresh".to_string(),
            refresh_expired: 86400,
        };
        let cloned = vo.clone();

        assert_eq!(vo.access_token, cloned.access_token);
        assert_eq!(vo.access_expired, cloned.access_expired);
        assert_eq!(vo.refresh_token, cloned.refresh_token);
        assert_eq!(vo.refresh_expired, cloned.refresh_expired);
    }
}
