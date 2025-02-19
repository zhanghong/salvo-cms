use sea_orm::DatabaseConnection;

use cms_core::{
    domain::{handle_ok, HandleResult},
    enums::PlatformEnum,
    service::AuthService,
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
        let res = AuthService::generate_access_token(1, "admin")?;
        println!("res: {:?}", res);
        handle_ok(true)
    }
}
