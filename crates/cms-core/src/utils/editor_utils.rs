use crate::domain::dto::EditorCurrentDTO;
use salvo::prelude::*;

/// 从 Depot 中获取当前编辑器的状态
///
/// # Parameters
///
/// * `depot`: 一个引用 Depot 的引用，用于存储和检索请求相关的数据
///
/// # Returns
///
/// * 返回 `EditorCurrentDTO` 实例，如果 Depot 中没有当前编辑器的状态或发生错误，则返回一个空的 `EditorCurrentDTO` 实例
pub fn get_current(depot: &Depot) -> EditorCurrentDTO {
    let res = depot.get::<EditorCurrentDTO>("current_editor");
    if res.is_err() {
        EditorCurrentDTO::empty()
    } else {
        let opt = res.unwrap();
        let editor = opt.to_owned();
        editor
    }
}

#[cfg(test)]
mod tests {
    use uuid::Uuid;

    use crate::enums::EditorTypeEnum;

    use super::*;

    #[test]
    fn test_get_current_when_not_present() {
        // Arrange: 创建一个空的 Depot
        let depot = Depot::new();

        // Act
        let result = get_current(&depot);

        // Assert: 应该返回 empty 实例
        assert_eq!(result, EditorCurrentDTO::empty())
    }

    #[test]
    fn test_get_current_when_present() {
        // Arrange: 创建一个带值的 Depot
        let mut depot = Depot::new();
        let uuid = Uuid::parse_str("f904857e-706f-44a7-b917-998c28ec9ca8").unwrap();
        let editor = EditorCurrentDTO {
            editor_id: uuid,
            editor_type: EditorTypeEnum::Admin,
        };
        depot.insert("current_editor", editor.clone());

        // Act
        let result = get_current(&depot);

        // Assert: 返回的值应与插入的一致
        assert_eq!(result, editor);
    }

    #[test]
    fn test_get_current_when_error_occurs() {
        // Arrange: 插入一个类型冲突的值，触发错误
        let mut depot = Depot::new();
        depot.insert("current_editor", "I am not an EditorCurrentDTO");

        // Act
        let result = get_current(&depot);

        // Assert: 应该忽略错误并返回 empty
        assert_eq!(result, EditorCurrentDTO::empty())
    }
}
