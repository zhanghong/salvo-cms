use serde::{Deserialize, Serialize};

use crate::enums::PrimaryIdEnum;

use super::EditorCurrentDTO;

/// Model Logic Delete DTO
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Default)]
pub struct ModelLogicDeleteDTO {
    pub id: PrimaryIdEnum,

    /// 编辑用户
    pub editor: EditorCurrentDTO,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::enums::PrimaryIdEnum;
    use serde_json;
    use uuid::Uuid;

    #[test]
    fn test_default_initialization() {
        let dto = ModelLogicDeleteDTO::default();
        assert_eq!(dto.id, PrimaryIdEnum::default());
        assert_eq!(dto.editor, EditorCurrentDTO::empty());
    }

    #[test]
    fn test_clone_equality() {
        let dto = ModelLogicDeleteDTO {
            id: PrimaryIdEnum::BigInt(123), // 假设 PrimaryIdEnum 有一个 new 方法
            editor: EditorCurrentDTO::empty(),
        };
        let cloned = dto.clone();
        assert_eq!(dto, cloned);
    }

    #[test]
    fn test_partial_eq_inequality() {
        let dto1 = ModelLogicDeleteDTO {
            id: PrimaryIdEnum::BigInt(1),
            editor: EditorCurrentDTO::empty(),
        };
        let dto2 = ModelLogicDeleteDTO {
            id: PrimaryIdEnum::BigInt(3),
            editor: EditorCurrentDTO::empty(),
        };
        assert_ne!(dto1, dto2);
    }

    #[test]
    fn test_serialize_deserialize() {
        let dto = ModelLogicDeleteDTO {
            id: PrimaryIdEnum::Uuid(Uuid::new_v4()),
            editor: EditorCurrentDTO::empty(),
        };

        let serialized = serde_json::to_string(&dto).unwrap();
        let deserialized: ModelLogicDeleteDTO = serde_json::from_str(&serialized).unwrap();

        assert_eq!(dto, deserialized);
    }
}
