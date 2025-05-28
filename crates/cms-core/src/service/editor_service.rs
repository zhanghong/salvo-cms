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
    pub async fn batch_load_by_uuids_str(
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

#[cfg(test)]
mod tests {
    use super::*;

    use crate::domain::entity::editor::Model as EditorModel;
    use crate::fixture::{config::app::FakerAppState, model::editors};

    #[tokio::test]
    async fn test_load_by_uuid_str() {
        let editor_system = editors::faker_model_by_name(editors::EDITOR_NAME_SYSTEM);
        let editor_admin = editors::faker_model_by_name(editors::EDITOR_NAME_ADMIN);

        let mut state = FakerAppState::init();
        state.db = MockDatabase::new(DatabaseBackend::Postgres)
            .append_query_results([
                // nil uuid not query
                Vec::<EditorModel>::new(),
                vec![editor_admin.clone()],
            ])
            .append_query_results([vec![editor_system.clone()]])
            .into_connection();

        let uuid = Uuid::nil().to_string();
        let result = EditorService::load_by_uuid_str(uuid.as_str(), &state).await;
        assert!(result.unwrap().is_none());

        let uuid = editor_system.id.clone().to_string();
        let result = EditorService::load_by_uuid_str(uuid.as_str(), &state).await;
        assert!(result.unwrap().is_none());

        let uuid = Uuid::new_v4().to_string();
        let result = EditorService::load_by_uuid_str(uuid.as_str(), &state).await;
        assert!(result.unwrap().is_none());

        let admin_uuid = editor_admin.id.clone().to_string();
        let editor_vo: EditorLoadVO = editor_admin.into();
        let result = EditorService::load_by_uuid_str(admin_uuid.as_str(), &state).await;
        assert_eq!(result.unwrap().unwrap(), editor_vo);
    }

    #[tokio::test]
    async fn test_load_by_uuid() {
        let editor_system = editors::faker_model_by_name(editors::EDITOR_NAME_SYSTEM);
        let editor_admin = editors::faker_model_by_name(editors::EDITOR_NAME_ADMIN);

        let mut state = FakerAppState::init();
        state.db = MockDatabase::new(DatabaseBackend::Postgres)
            .append_query_results([
                // nil uuid not query
                Vec::<EditorModel>::new(),
                vec![editor_admin.clone()],
            ])
            .append_query_results([vec![editor_system.clone()]])
            .into_connection();

        let uuid = Uuid::nil();
        let result = EditorService::load_by_uuid(&uuid, &state).await;
        assert!(result.unwrap().is_none());

        let result = EditorService::load_by_uuid(&(editor_system.id), &state).await;
        assert!(result.unwrap().is_none());

        let uuid = Uuid::new_v4();
        let result = EditorService::load_by_uuid(&uuid, &state).await;
        assert!(result.unwrap().is_none());

        let admin_uuid = editor_admin.id.clone();
        let editor_vo: EditorLoadVO = editor_admin.into();
        let result = EditorService::load_by_uuid(&admin_uuid, &state).await;
        assert_eq!(result.unwrap().unwrap(), editor_vo);
    }

    #[tokio::test]
    async fn test_batch_load_by_uuids_str() {
        let editor_system = editors::faker_model_by_name(editors::EDITOR_NAME_SYSTEM);
        let editor_admin = editors::faker_model_by_name(editors::EDITOR_NAME_ADMIN);
        let editor_guest = editors::faker_model_by_name(editors::EDITOR_NAME_GUEST);

        let mut state = FakerAppState::init();
        state.db = MockDatabase::new(DatabaseBackend::Postgres)
            .append_query_results([
                // nil uuid not query
                vec![editor_admin.clone(), editor_system.clone()],
            ])
            .into_connection();

        let uuids = Vec::<&str>::new();
        let result = EditorService::batch_load_by_uuids_str(&uuids, &state).await;
        assert!(result.unwrap().is_empty());

        let uuids: Vec<String> = vec![
            Uuid::nil(),
            editor_system.id.clone(),
            editor_admin.id.clone(),
            editor_guest.id.clone(),
            Uuid::new_v4(),
        ]
        .iter()
        .map(|uuid| uuid.to_string())
        .collect();
        let uuids: Vec<&str> = uuids.iter().map(|uuid| uuid.as_str()).collect();
        let result = EditorService::batch_load_by_uuids_str(&uuids, &state).await;
        assert!(result.is_ok());
        let map = result.unwrap();
        assert_eq!(map.len(), 2);
        assert_eq!(
            map.get(&editor_admin.id.to_string()).unwrap(),
            &editor_admin.into()
        );
        assert_eq!(
            map.get(&editor_system.id.to_string()).unwrap(),
            &editor_system.into()
        );
    }

    #[tokio::test]
    async fn test_batch_load_by_uuids() {
        let editor_system = editors::faker_model_by_name(editors::EDITOR_NAME_SYSTEM);
        let editor_admin = editors::faker_model_by_name(editors::EDITOR_NAME_ADMIN);
        let editor_guest = editors::faker_model_by_name(editors::EDITOR_NAME_GUEST);

        let mut state = FakerAppState::init();
        state.db = MockDatabase::new(DatabaseBackend::Postgres)
            .append_query_results([
                // nil uuid not query
                vec![editor_admin.clone(), editor_system.clone()],
            ])
            .into_connection();

        let uuids = Vec::<Uuid>::new();
        let result = EditorService::batch_load_by_uuids(&uuids, &state).await;
        assert!(result.unwrap().is_empty());

        let uuids = vec![
            Uuid::nil(),
            editor_system.id.clone(),
            editor_admin.id.clone(),
            editor_guest.id.clone(),
            Uuid::new_v4(),
        ];
        let result = EditorService::batch_load_by_uuids(&uuids, &state).await;
        assert!(result.is_ok());
        let map = result.unwrap();
        assert_eq!(map.len(), 2);
        assert_eq!(
            map.get(&editor_admin.id.to_string()).unwrap(),
            &editor_admin.into()
        );
        assert_eq!(
            map.get(&editor_system.id.to_string()).unwrap(),
            &editor_system.into()
        );
    }
}
