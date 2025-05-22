use serde::{Deserialize, Serialize};

use super::EditorCurrentDTO;
use crate::enums::PrimaryIdEnum;

/// Model View DTO
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Default)]
pub struct ModelViewDTO<T> {
    /// 主键
    pub id: PrimaryIdEnum,

    /// 编辑用户
    pub editor: EditorCurrentDTO,

    /// 是否启用
    pub enabled: Option<bool>,

    /// 加载关联数据
    pub load_models: Option<Vec<T>>,
}

#[cfg(test)]
mod tests {
    use super::*;

    // 使用 i32 作为泛型参数来测试 ModelViewDTO
    type TestModelViewDTO = ModelViewDTO<i32>;

    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Default)]
    struct DummyStruct {
        value: i32,
    }

    // 使用 DummyStruct 作为泛型参数再次测试
    type ComplexModelViewDTO = ModelViewDTO<DummyStruct>;

    // 构建一个测试用的 ModelViewDTO 实例
    fn get_test_dto() -> TestModelViewDTO {
        ModelViewDTO {
            id: PrimaryIdEnum::default(),
            editor: EditorCurrentDTO::default(),
            enabled: Some(true),
            load_models: Some(vec![1, 2, 3]),
        }
    }

    #[test]
    fn test_serialize_deserialize() {
        let dto = get_test_dto();
        let serialized = serde_json::to_string(&dto).unwrap();
        let deserialized: TestModelViewDTO = serde_json::from_str(&serialized).unwrap();
        assert_eq!(dto, deserialized);
    }

    #[test]
    fn test_partial_eq() {
        let dto1 = get_test_dto();
        let mut dto2 = get_test_dto();
        assert_eq!(dto1, dto2);

        dto2.enabled = Some(false);
        assert_ne!(dto1, dto2);
    }

    #[test]
    fn test_clone() {
        let dto1 = get_test_dto();
        let dto2 = dto1.clone();
        assert_eq!(dto1, dto2);
    }

    #[test]
    fn test_default() {
        let dto: TestModelViewDTO = ModelViewDTO::default();
        assert_eq!(dto.id, PrimaryIdEnum::default());
        assert_eq!(dto.editor, EditorCurrentDTO::default());
        assert_eq!(dto.enabled, None);
        assert_eq!(dto.load_models, None);
    }

    #[test]
    fn test_generic_with_complex_type() {
        let dto: ComplexModelViewDTO = ModelViewDTO {
            id: PrimaryIdEnum::default(),
            editor: EditorCurrentDTO::default(),
            enabled: Some(false),
            load_models: Some(vec![DummyStruct { value: 10 }, DummyStruct { value: 20 }]),
        };

        let serialized = serde_json::to_string(&dto).unwrap();
        let deserialized: ComplexModelViewDTO = serde_json::from_str(&serialized).unwrap();
        assert_eq!(dto, deserialized);
    }
}
