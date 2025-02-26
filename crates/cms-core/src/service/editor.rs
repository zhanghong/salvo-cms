use sea_orm::*;
use std::collections::HashMap;

use crate::{
    config::AppState,
    domain::{
        entity::editor::{Column as EditorColumn, Entity as EditorEntity},
        handle_ok,
        vo::EditorLoadVO,
        HandleResult,
    },
};

pub struct EditorService {}

impl EditorService {
    /// 查询关联的单个记录
    pub async fn load_by_id(id: i64, state: &AppState) -> HandleResult<Option<EditorLoadVO>> {
        let db = &state.db;
        let opt = EditorEntity::find_by_id(id).one(db).await?;
        if let Some(editor) = opt {
            handle_ok(Some(editor.into()))
        } else {
            handle_ok(None)
        }
    }

    /// 批量查询关联的记录
    pub async fn batch_load_by_ids(
        ids: &Vec<i64>,
        state: &AppState,
    ) -> HandleResult<HashMap<i64, EditorLoadVO>> {
        let filted_ids: Vec<i64> = ids.into_iter().filter(|&&id| id > 0).cloned().collect();
        if filted_ids.is_empty() {
            return handle_ok(HashMap::<i64, EditorLoadVO>::new());
        }

        let db = &state.db;
        let models = EditorEntity::find()
            .filter(EditorColumn::Id.is_in(filted_ids))
            .all(db)
            .await?;

        let map: HashMap<i64, EditorLoadVO> = models
            .into_iter()
            .map(|model| (model.id, model.into()))
            .collect();

        handle_ok(map)
    }
}
