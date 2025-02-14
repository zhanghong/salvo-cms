use chrono::Local;
use cms_core::error::AppError;
use sea_orm::*;

use cms_core::domain::{handle_ok, HandleResult};
use cms_core::enums::PlatformEnum;
use cms_core::utils::{encrypt::encrypt_password, random};
use sea_orm::DatabaseConnection;

use crate::domain::dto::UserStoreDTO;
use crate::domain::entity::user::{
    ActiveModel as UserActiveModel, Column as UserColumn, Entity as UserEntity, Model as UserModel,
};
use crate::enums::{GenderEnum, UserTypeEnum};

pub struct UserService {}

const RAND_SALT_LENGTH: usize = 5;
const RAND_NO_LENGTH: usize = 10;

impl UserService {
    pub async fn create(
        platform: PlatformEnum,
        dto: &UserStoreDTO,
        db: &DatabaseConnection,
    ) -> HandleResult<UserModel> {
        let mut model = UserActiveModel {
            ..Default::default()
        };

        let id: u64 = 0;
        let no = dto.no.clone();
        let no = match no {
            Some(str) => str,
            None => {
                let rand_str = random::alpha_string(RAND_NO_LENGTH);
                format!("U{}", rand_str)
            }
        };
        model.no = Set(no);

        let name = dto.name.clone();
        if name.is_some() {
            let name = name.unwrap();
            let is_exists = Self::is_column_exist(
                id,
                UserColumn::Name,
                sea_orm::Value::from(name.to_owned()),
                db,
            )
            .await?;
            if is_exists {
                let err = AppError::BadRequest(String::from("用户名已存在"));
                return Err(err);
            }
            model.name = Set(name);
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
            let user_type = user_type.unwrap();
            match platform {
                PlatformEnum::Open => {
                    let type_name = UserTypeEnum::Member.to_string();
                    model.user_type = Set(type_name);
                }
                _ => {
                    model.user_type = Set(user_type);
                }
            };
        }

        if dto.gender.is_some() {
            let gender = dto.gender.clone().unwrap();
            match gender {
                GenderEnum::None => {}
                _ => {
                    model.gender = Set(gender.as_value());
                }
            }
        }

        let phone = dto.phone.clone();
        if phone.is_some() {
            let phone = phone.unwrap();
            let is_exists = Self::is_column_exist(
                id,
                UserColumn::Phone,
                sea_orm::Value::from(phone.to_owned()),
                db,
            )
            .await?;
            if is_exists {
                let err = AppError::BadRequest(String::from("手机号已存在"));
                return Err(err);
            }
            model.phone = Set(phone);
        }

        let avatar_path: Option<String> = dto.avatar_path.clone();
        if avatar_path.is_some() {
            model.avatar_path = Set(avatar_path.unwrap());
        }

        let email: Option<String> = dto.email.clone();
        if email.is_some() {
            let email = email.unwrap();
            let is_exists = Self::is_column_exist(
                id,
                UserColumn::Email,
                sea_orm::Value::from(email.to_owned()),
                db,
            )
            .await?;
            if is_exists {
                let err = AppError::BadRequest(String::from("邮箱已存在"));
                return Err(err);
            }
            model.email = Set(email);
        }

        let password = dto.password.clone();
        if password.is_some() {
            let password = password.unwrap();
            let password_str = password.as_str();
            let confirm_password = dto.confirm_password.clone();
            if confirm_password.is_none() {
                let err = AppError::BadRequest(String::from("确认密码不能为空"));
                return Err(err);
            } else if !confirm_password.unwrap().eq(password_str) {
                let err = AppError::BadRequest(String::from("两次输入的密码不一致"));
                return Err(err);
            }

            let salt = random::alpha_string(RAND_SALT_LENGTH);
            let password = encrypt_password(salt.as_str(), password_str);
            println!("password str: {}", password_str);
            println!("password: {}", password);
            model.salt = Set(salt);
            model.password = Set(password);
        }

        let data_source_id = match dto.data_source_id {
            Some(id) => id,
            None => 0,
        };
        model.data_source_id = Set(data_source_id);

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

    async fn is_column_exist(
        id: u64,
        column: UserColumn,
        value: sea_orm::Value,
        db: &DatabaseConnection,
    ) -> HandleResult<bool> {
        let count = UserEntity::find()
            .filter(column.eq(value))
            .filter(UserColumn::Id.ne(id))
            .count(db)
            .await?;

        handle_ok(count > 0)
    }
}
