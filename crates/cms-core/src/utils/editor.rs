use crate::domain::dto::EditorCurrent;
use salvo::prelude::*;

/// 从 Depot 中获取当前编辑器的状态
///
/// # Parameters
///
/// * `depot`: 一个引用 Depot 的引用，用于存储和检索请求相关的数据
///
/// # Returns
///
/// * 返回 `EditorCurrent` 实例，如果 Depot 中没有当前编辑器的状态或发生错误，则返回一个空的 `EditorCurrent` 实例
pub fn get_current(depot: &Depot) -> EditorCurrent {
    let res = depot.get::<EditorCurrent>("current_editor");
    if res.is_err() {
        EditorCurrent::empty()
    } else {
        let opt = res.unwrap();
        let editor = opt.to_owned();
        editor
    }
}
