use uuid::Uuid;

use crate::domain::entity::editor::Model as EditorModel;

pub const EDITOR_NAME_SYSTEM: &str = "system";
pub const EDITOR_NAME_ADMIN: &str = "admin";
pub const EDITOR_NAME_GUEST: &str = "guest";
pub fn faker_uuid_by_name(name: &str) -> Uuid {
    match name {
        EDITOR_NAME_SYSTEM => Uuid::parse_str("00000000-0000-0000-0000-000000000000").unwrap(),
        EDITOR_NAME_ADMIN => Uuid::parse_str("b94a84ea-592c-4eb2-0001-001000000011").unwrap(),
        EDITOR_NAME_GUEST => Uuid::parse_str("b94a84ea-592c-4eb2-0001-001000000021").unwrap(),
        _ => Uuid::nil(),
    }
}

pub fn faker_model_by_name(name: &str) -> EditorModel {
    let uuid = faker_uuid_by_name(name);
    EditorModel {
        id: uuid,
        no: name.to_owned(),
        name: name.to_owned(),
        phone: "".to_owned(),
        email: format!("{}@test.com", name),
        avatar_path: "".to_owned(),
    }
}
