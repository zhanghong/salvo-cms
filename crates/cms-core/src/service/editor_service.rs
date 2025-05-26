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
    /// Load By UUID str
    pub async fn load_by_uuid_str(
        str: &str,
        state: &AppState,
    ) -> HandleResult<Option<EditorLoadVO>> {
        let uuid = Uuid::parse_str(str).unwrap_or(Uuid::nil());
        Self::load_by_uuid(&uuid, state).await
    }

    /// Load by uuid
    pub async fn load_by_uuid(uuid: &Uuid, state: &AppState) -> HandleResult<Option<EditorLoadVO>> {
        if uuid.is_nil() {
            return handle_ok(None);
        }
        let db = &state.db;
        let opt = EditorEntity::find_by_id(*uuid).one(db).await?;
        if let Some(editor) = opt {
            handle_ok(Some(editor.into()))
        } else {
            handle_ok(None)
        }
    }

    /// Batch load by uuid str vec
    pub async fn batch_load_by_ids(
        ids: &Vec<&str>,
        state: &AppState,
    ) -> HandleResult<HashMap<String, EditorLoadVO>> {
        let filted_ids: Vec<Uuid> = ids
            .into_iter()
            .map(|id| Uuid::parse_str(id).unwrap_or(Uuid::nil()))
            .filter(|id| Uuid::nil() != *id)
            .collect();

        Self::batch_load_by_uuids(&filted_ids, state).await
    }

    /// Batch load by uuid vec
    pub async fn batch_load_by_uuids(
        uuids: &Vec<Uuid>,
        state: &AppState,
    ) -> HandleResult<HashMap<String, EditorLoadVO>> {
        let filted_ids: Vec<Uuid> = uuids
            .into_iter()
            .filter(|id| Uuid::nil() != **id)
            .map(|id| *id)
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
