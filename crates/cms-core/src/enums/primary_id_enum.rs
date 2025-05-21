use salvo::oapi::ToSchema;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::fmt;
use std::str::FromStr;
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Eq, Hash, ToSchema)]
#[salvo(schema(name = "Core::Enum::PrimaryIdEnum"))]
pub enum PrimaryIdEnum {
    BigInt(i64),
    Uuid(Uuid),
    Nil,
}

impl Default for PrimaryIdEnum {
    fn default() -> Self {
        Self::Nil
    }
}

impl PrimaryIdEnum {
    /// 尝试将字符串解析为 PrimaryIdEnum
    pub fn from_str(s: &str) -> Self {
        let s = s.trim();
        let len = s.len();

        if len == 0 {
            return Self::Nil;
        }

        // 检查是否是纯数字且不超过 i64 最大长度
        if len <= 19 && s.chars().all(|c| c.is_ascii_digit()) {
            match s.parse::<i64>() {
                Ok(id) => return Self::BigInt(id),
                Err(e) if e.to_string().contains("number too large") => {
                    // 可选：记录日志或处理溢出
                    return Self::Nil;
                }
                _ => {}
            }
        }

        if let Ok(uuid) = Uuid::from_str(s) {
            return Self::Uuid(uuid);
        }

        Self::Nil
    }

    pub fn active_int_id(&self) -> Option<i64> {
        match self {
            Self::BigInt(id) if *id > 0 => Some(*id),
            _ => None,
        }
    }

    pub fn active_uuid_id(&self) -> Option<Uuid> {
        match self {
            Self::Uuid(uuid) if !Uuid::is_nil(uuid) => Some(*uuid),
            _ => None,
        }
    }
}

// 序列化实现
impl Serialize for PrimaryIdEnum {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::BigInt(id) => id.serialize(serializer),
            Self::Uuid(uuid) => uuid.to_string().serialize(serializer),
            Self::Nil => Option::<String>::None.serialize(serializer),
        }
    }
}

impl<'de> Deserialize<'de> for PrimaryIdEnum {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_any(IdVisitor)
    }
}

struct IdVisitor;

impl<'de> serde::de::Visitor<'de> for IdVisitor {
    type Value = PrimaryIdEnum;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("a string or integer representing a model ID")
    }

    fn visit_str<E>(self, value: &str) -> Result<PrimaryIdEnum, E>
    where
        E: serde::de::Error,
    {
        Ok(PrimaryIdEnum::from_str(value))
    }

    fn visit_i64<E>(self, value: i64) -> Result<PrimaryIdEnum, E>
    where
        E: serde::de::Error,
    {
        if value < 0 {
            return Err(E::invalid_value(
                serde::de::Unexpected::Signed(value),
                &"non-negative integer",
            ));
        }
        Ok(PrimaryIdEnum::BigInt(value))
    }

    fn visit_u64<E>(self, value: u64) -> Result<PrimaryIdEnum, E>
    where
        E: serde::de::Error,
    {
        if value > i64::MAX as u64 {
            return Err(E::invalid_value(
                serde::de::Unexpected::Unsigned(value),
                &"value exceeds i64::MAX",
            ));
        }
        Ok(PrimaryIdEnum::BigInt(value as i64))
    }

    fn visit_none<E>(self) -> Result<PrimaryIdEnum, E>
    where
        E: serde::de::Error,
    {
        Ok(PrimaryIdEnum::Nil)
    }

    fn visit_unit<E>(self) -> Result<PrimaryIdEnum, E>
    where
        E: serde::de::Error,
    {
        Ok(PrimaryIdEnum::Nil)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json;
    use uuid::Uuid;

    #[test]
    fn test_from_str_empty() {
        assert_eq!(PrimaryIdEnum::from_str(""), PrimaryIdEnum::Nil);
    }

    #[test]
    fn test_from_str_number() {
        assert_eq!(PrimaryIdEnum::from_str("123"), PrimaryIdEnum::BigInt(123));
        assert_eq!(PrimaryIdEnum::from_str("0"), PrimaryIdEnum::BigInt(0));
        assert_eq!(PrimaryIdEnum::from_str("-123"), PrimaryIdEnum::Nil); // 负数不接受
    }

    #[test]
    fn test_from_str_large_number() {
        let large = "9223372036854775807"; // i64::MAX
        assert_eq!(
            PrimaryIdEnum::from_str(large),
            PrimaryIdEnum::BigInt(i64::MAX)
        );

        let too_large = "18446744073709551615"; // u64::MAX
        assert_eq!(PrimaryIdEnum::from_str(too_large), PrimaryIdEnum::Nil);
    }

    #[test]
    fn test_from_str_uuid() {
        let uuid_str = "550e8400-e29b-41d4-a716-446655440000";
        let uuid = Uuid::parse_str(uuid_str).unwrap();
        assert_eq!(PrimaryIdEnum::from_str(uuid_str), PrimaryIdEnum::Uuid(uuid));
    }

    #[test]
    fn test_from_str_invalid() {
        assert_eq!(PrimaryIdEnum::from_str("invalid"), PrimaryIdEnum::Nil);
        assert_eq!(PrimaryIdEnum::from_str("abc123"), PrimaryIdEnum::Nil);
    }

    #[test]
    fn test_active_int_id() {
        assert_eq!(PrimaryIdEnum::BigInt(1).active_int_id(), Some(1));
        assert_eq!(PrimaryIdEnum::BigInt(0).active_int_id(), None);
        assert_eq!(PrimaryIdEnum::Uuid(Uuid::new_v4()).active_int_id(), None);
        assert_eq!(PrimaryIdEnum::Nil.active_int_id(), None);
    }

    #[test]
    fn test_active_uuid_id() {
        let uuid = Uuid::new_v4();
        assert_eq!(PrimaryIdEnum::Uuid(uuid).active_uuid_id(), Some(uuid));
        assert_eq!(PrimaryIdEnum::Uuid(Uuid::nil()).active_uuid_id(), None);
        assert_eq!(PrimaryIdEnum::BigInt(123).active_uuid_id(), None);
        assert_eq!(PrimaryIdEnum::Nil.active_uuid_id(), None);
    }

    #[test]
    fn test_serialize_bigint() {
        let id = PrimaryIdEnum::BigInt(123);
        let json = serde_json::to_string(&id).unwrap();
        assert_eq!(json, "123");
    }

    #[test]
    fn test_serialize_uuid() {
        let uuid = Uuid::parse_str("550e8400-e29b-41d4-a716-446655440000").unwrap();
        let id = PrimaryIdEnum::Uuid(uuid);
        let json = serde_json::to_string(&id).unwrap();
        assert_eq!(json, "\"550e8400-e29b-41d4-a716-446655440000\"");
    }

    #[test]
    fn test_serialize_nil() {
        let id = PrimaryIdEnum::Nil;
        let json = serde_json::to_string(&id).unwrap();
        assert_eq!(json, "null");
    }

    #[test]
    fn test_deserialize_from_number() {
        let json = "123";
        let id: PrimaryIdEnum = serde_json::from_str(json).unwrap();
        assert_eq!(id, PrimaryIdEnum::BigInt(123));
    }

    #[test]
    fn test_deserialize_from_negative_number() {
        let json = "-123";
        let result: Result<PrimaryIdEnum, _> = serde_json::from_str(json);
        assert!(result.is_err());
    }

    #[test]
    fn test_deserialize_from_large_u64() {
        let json = "18446744073709551615";
        let result: Result<PrimaryIdEnum, _> = serde_json::from_str(json);
        assert!(result.is_err());
    }

    #[test]
    fn test_deserialize_from_uuid_string() {
        let json = "\"550e8400-e29b-41d4-a716-446655440000\"";
        let id: PrimaryIdEnum = serde_json::from_str(json).unwrap();
        let expected =
            PrimaryIdEnum::Uuid(Uuid::parse_str("550e8400-e29b-41d4-a716-446655440000").unwrap());
        assert_eq!(id, expected);
    }

    #[test]
    fn test_deserialize_from_null() {
        let json = "null";
        let id: PrimaryIdEnum = serde_json::from_str(json).unwrap();
        assert_eq!(id, PrimaryIdEnum::Nil);
    }
}
