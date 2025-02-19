use sea_orm::DatabaseConnection;

use cms_core::{
    domain::{handle_ok, HandleResult},
    enums::PlatformEnum,
};

use crate::domain::dto::LoginStoreDTO;

pub struct LoginService {}

impl LoginService {
    pub async fn store(
        _platform: &PlatformEnum,
        dto: &LoginStoreDTO,
        _db: &DatabaseConnection,
    ) -> HandleResult<bool> {
        println!("dto: {:?}", dto);
        handle_ok(true)
    }
}
