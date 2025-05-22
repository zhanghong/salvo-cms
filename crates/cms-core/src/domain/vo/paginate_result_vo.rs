use salvo::oapi::ToSchema;
use serde::{Deserialize, Serialize};

use crate::utils::parameter_utils::{page_no_default, page_size_default};

/// Paginate Result VO
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, ToSchema)]
#[salvo(schema(name = "Core::Base::PaginateResultVO"))]
pub struct PaginateResultVO<T: Serialize> {
    /// 当前页码
    #[serde(default = "page_no_default")]
    #[salvo(schema(required = true, nullable = false, value_type = i64, minimum = 1, default = 1, example = 5))]
    pub current_page: u64,

    /// 每页显示条数
    #[serde(default = "page_size_default")]
    #[salvo(schema(required = true, nullable = false, value_type = i64, minimum = 1, maximum = 100, example = 50))]
    pub page_size: u64,

    /// 总条数
    #[salvo(schema(required = true, nullable = false, value_type = i64, minimum = 0, example = 50))]
    pub total: u64,

    #[salvo(schema(required = true, nullable = false, value_type = Vec<Object>))]
    pub list: Vec<T>,
}

impl<T: Serialize> Default for PaginateResultVO<T> {
    fn default() -> Self {
        let current_page = page_no_default();
        let page_size = page_size_default();
        Self {
            current_page,
            page_size,
            total: 0,
            list: Vec::new(),
        }
    }
}

#[cfg(test)]
mod tests {
    use serde_json::json;
    use uuid::Uuid;

    use super::*;

    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Default)]
    struct MockModel {
        id: Uuid,
        name: String,
    }

    fn mock_model_list() -> Vec<MockModel> {
        let mut list = Vec::new();
        for i in 1..=10 {
            list.push(MockModel {
                id: Uuid::new_v4(),
                name: format!("User {}", i),
            });
        }
        list
    }

    #[test]
    fn test_paginate_result_vo_serialize() {
        let models = mock_model_list();
        let vo: PaginateResultVO<MockModel> = PaginateResultVO {
            current_page: 5,
            page_size: 50,
            total: 50,
            list: models.clone(),
        };

        let expected_json = json!({
            "current_page": 5,
            "page_size": 50,
            "total": 50,
            "list": models
        });

        let serialized = serde_json::to_value(&vo).unwrap();
        assert_eq!(serialized, expected_json);
    }

    #[test]
    fn test_paginate_result_vo_deserialize() {
        let json_data = json!({
            "current_page": 3,
            "page_size": 20,
            "total": 100,
            "list": ["a", "b", "c"]
        });

        let deserialized: PaginateResultVO<String> = serde_json::from_value(json_data).unwrap();

        assert_eq!(deserialized.current_page, 3);
        assert_eq!(deserialized.page_size, 20);
        assert_eq!(deserialized.total, 100);
        assert_eq!(deserialized.list, vec!["a", "b", "c"]);
    }

    #[test]
    fn test_paginate_result_vo_default() {
        let default_vo: PaginateResultVO<String> = PaginateResultVO::default();

        assert_eq!(default_vo.current_page, page_no_default());
        assert_eq!(default_vo.page_size, page_size_default());
        assert_eq!(default_vo.total, 0);
        assert!(default_vo.list.is_empty());
    }

    #[test]
    fn test_paginate_result_vo_min_current_page() {
        let vo: PaginateResultVO<MockModel> = PaginateResultVO {
            current_page: 1,
            page_size: 10,
            total: 0,
            list: vec![],
        };
        let json = serde_json::to_value(&vo).unwrap();
        assert_eq!(json["current_page"], 1);
    }

    #[test]
    fn test_paginate_result_vo_min_page_size() {
        let vo: PaginateResultVO<MockModel> = PaginateResultVO {
            current_page: 1,
            page_size: 1,
            total: 0,
            list: vec![],
        };
        let json = serde_json::to_value(&vo).unwrap();
        assert_eq!(json["page_size"], 1);
    }

    #[test]
    fn test_paginate_result_vo_max_page_size() {
        let vo: PaginateResultVO<MockModel> = PaginateResultVO {
            current_page: 1,
            page_size: 100,
            total: 0,
            list: vec![],
        };
        let json = serde_json::to_value(&vo).unwrap();
        assert_eq!(json["page_size"], 100);
    }

    #[test]
    fn test_paginate_result_vo_zero_total() {
        let vo: PaginateResultVO<MockModel> = PaginateResultVO {
            current_page: 1,
            page_size: 10,
            total: 0,
            list: vec![],
        };
        let json = serde_json::to_value(&vo).unwrap();
        assert_eq!(json["total"], 0);
    }

    #[test]
    fn test_paginate_result_vo_generic_support() {
        let vo: PaginateResultVO<i32> = PaginateResultVO {
            current_page: 1,
            page_size: 2,
            total: 3,
            list: vec![100, 200],
        };

        let json = serde_json::to_value(&vo).unwrap();
        let deserialized: PaginateResultVO<i32> = serde_json::from_value(json).unwrap();

        assert_eq!(deserialized.current_page, 1);
        assert_eq!(deserialized.page_size, 2);
        assert_eq!(deserialized.total, 3);
        assert_eq!(deserialized.list, vec![100, 200]);
    }
}
