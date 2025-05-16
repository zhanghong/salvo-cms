use sea_orm::*;
use std::collections::HashMap;
use uuid::Uuid;

use crate::{
    config::AppState,
    domain::{
        HandleResult,
        entity::editor::{Column as EditorColumn, Entity as EditorEntity},
        handle_ok,
        vo::EditorLoadVO,
    },
};

pub struct EditorService {}

impl EditorService {
    /// 查询关联的单个记录
    pub async fn load_by_id(id: &str, state: &AppState) -> HandleResult<Option<EditorLoadVO>> {
        let db = &state.db;
        let id = Uuid::parse_str(id);
        if id.is_err() {
            return handle_ok(None);
        }
        let opt = EditorEntity::find_by_id(id.unwrap()).one(db).await?;
        if let Some(editor) = opt {
            handle_ok(Some(editor.into()))
        } else {
            handle_ok(None)
        }
    }

    /// 批量查询关联的记录
    pub async fn batch_load_by_ids(
        ids: &Vec<&str>,
        state: &AppState,
    ) -> HandleResult<HashMap<String, EditorLoadVO>> {
        let filted_ids: Vec<Uuid> = ids
            .into_iter()
            .map(|id| Uuid::parse_str(id).unwrap_or(Uuid::nil()))
            .filter(|id| Uuid::nil() != *id)
            .collect();
        if filted_ids.is_empty() {
            return handle_ok(HashMap::<String, EditorLoadVO>::new());
        }

        let db = &state.db;
        let models = EditorEntity::find()
            .filter(EditorColumn::Id.is_in(filted_ids))
            .all(db)
            .await?;

        let map: HashMap<String, EditorLoadVO> = models
            .into_iter()
            .map(|model| (model.id.to_string(), model.into()))
            .collect();

        handle_ok(map)
    }
}
