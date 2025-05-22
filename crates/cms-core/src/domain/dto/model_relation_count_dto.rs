// use sea_orm::FromQueryResult;
use serde::{Deserialize, Serialize};

use crate::enums::PrimaryIdEnum;

/// Model Reloation Count DTO
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Default)]
pub struct ModelRelationCountDTO {
    /// 关联主键
    pub relation_id: PrimaryIdEnum,

    /// 统计数量
    pub item_count: i16,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::enums::PrimaryIdEnum; // 假设 PrimaryIdEnum 已定义
    use serde_json;
    use uuid::Uuid;

    #[test]
    fn test_model_relation_count_dto_fields() {
        let uuid = Uuid::new_v4();
        let dto = ModelRelationCountDTO {
            relation_id: PrimaryIdEnum::Uuid(uuid.clone()),
            item_count: 42,
        };

        assert_eq!(dto.relation_id, PrimaryIdEnum::Uuid(uuid));
        assert_eq!(dto.item_count, 42);
    }

    #[test]
    fn test_model_relation_count_dto_default() {
        let dto = ModelRelationCountDTO::default();

        assert_eq!(dto.relation_id, PrimaryIdEnum::default());
        assert_eq!(dto.item_count, 0);
    }

    #[test]
    fn test_model_relation_count_dto_clone() {
        let dto1 = ModelRelationCountDTO {
            relation_id: PrimaryIdEnum::Uuid(Uuid::new_v4()),
            item_count: 78,
        };
        let dto2 = dto1.clone();

        assert_eq!(dto1, dto2);
    }

    #[test]
    fn test_model_relation_count_dto_partial_eq() {
        let uuid = Uuid::new_v4();
        let dto1 = ModelRelationCountDTO {
            relation_id: PrimaryIdEnum::Uuid(uuid.clone()),
            item_count: 10,
        };
        let dto2 = ModelRelationCountDTO {
            relation_id: PrimaryIdEnum::Uuid(uuid),
            item_count: 10,
        };

        assert_eq!(dto1, dto2);
    }

    #[test]
    fn test_model_relation_count_dto_serde_serialize_deserialize() {
        let dto = ModelRelationCountDTO {
            relation_id: PrimaryIdEnum::BigInt(101),
            item_count: 202,
        };

        let serialized = serde_json::to_string(&dto).unwrap();
        let deserialized: ModelRelationCountDTO = serde_json::from_str(&serialized).unwrap();

        assert_eq!(dto, deserialized);
    }
}
