use chrono::Local;
use cms_core::domain::{handle_ok, HandleResult};
use sea_orm::*;

use cms_core::enums::PlatformEnum;
use sea_orm::DatabaseConnection;

use crate::domain::dto::UserStoreDTO;
use crate::domain::entity::user::{ActiveModel as UserActiveModel, Model as UserModel};

pub struct UserService {}

impl UserService {
    pub async fn create(
        _platform: PlatformEnum,
        dto: &UserStoreDTO,
        db: &DatabaseConnection,
    ) -> HandleResult<UserModel> {
        let mut model = UserActiveModel {
            ..Default::default()
        };

        let name = dto.name.clone();
        if name.is_some() {
            model.name = Set(name.unwrap());
        }

        let realname = dto.realname.clone();
        if realname.is_some() {
            model.realname = Set(realname.unwrap());
        }

        let nickname = dto.nickname.clone();
        if nickname.is_some() {
            model.nickname = Set(nickname.unwrap());
        }

        let user_type = dto.user_type.clone();
        if user_type.is_some() {
            model.user_type = Set(user_type.unwrap());
        }

        let gender = match dto.gender {
            Some(id) => id,
            None => 0,
        };
        model.gender = Set(gender);

        let phone = dto.phone.clone();
        if phone.is_some() {
            model.phone = Set(phone.unwrap());
        }

        let avatar_path: Option<String> = dto.avatar_path.clone();
        if avatar_path.is_some() {
            model.avatar_path = Set(avatar_path.unwrap());
        }

        let email: Option<String> = dto.email.clone();
        if email.is_some() {
            model.email = Set(email.unwrap());
        }

        let data_source_id = match dto.data_source_id {
            Some(id) => id,
            None => 0,
        };
        model.data_source_id = Set(data_source_id);

        let password: Option<String> = dto.password.clone();
        if password.is_some() {
            model.password = Set(password.unwrap());
        }

        let is_authed = match dto.is_authed {
            Some(flag) => flag,
            None => false,
        };
        model.is_authed = Set(is_authed);

        let is_enabled = match dto.is_enabled {
            Some(flag) => flag,
            None => true,
        };
        model.is_enabled = Set(is_enabled);

        let is_test = match dto.is_test {
            Some(flag) => flag,
            None => false,
        };
        model.is_test = Set(is_test);

        let time = Local::now();
        model.created_at = Set(time);
        model.updated_at = Set(time);

        let model = model.save(db).await?;
        let model = model.try_into_model()?;

        handle_ok(model)
    }
}
