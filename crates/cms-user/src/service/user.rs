use chrono::Local;
use cms_core::error::AppError;
use sea_orm::*;

use cms_core::domain::{form::FieldValueUniqueForm, handle_ok, HandleResult};
use cms_core::enums::PlatformEnum;
use cms_core::utils::{encrypt::encrypt_password, random};
use sea_orm::DatabaseConnection;

use crate::domain::dto::UserStoreDTO;
use crate::domain::entity::user::{
    ActiveModel as UserActiveModel, Column as UserColumn, Entity as UserEntity, Model as UserModel,
};
use crate::domain::vo::UserFormOptionVO;
use crate::enums::{GenderEnum, UserTypeEnum};

pub struct UserService {}

const RAND_SALT_LENGTH: usize = 5;
const RAND_NO_LENGTH: usize = 10;

impl UserService {
    pub async fn store(
        platform: PlatformEnum,
        dto: &UserStoreDTO,
        db: &DatabaseConnection,
    ) -> HandleResult<UserModel> {
        let mut id: i64 = 0;
        let mut is_create = true;
        if dto.id.is_some() {
            id = dto.id.unwrap();
            is_create = false;
        }
        let mut model = if is_create {
            UserActiveModel {
                ..Default::default()
            }
        } else {
            let entity = UserEntity::find_by_id(id).one(db).await?.unwrap();
            entity.into()
        };

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

        let user_type = dto.types_list.clone();
        if user_type.is_some() {
            let list: Vec<UserTypeEnum> = user_type.unwrap();
            match platform {
                PlatformEnum::Open => {
                    let type_name = UserTypeEnum::Member.as_value();
                    model.user_type = Set(type_name);
                }
                _ => {
                    let type_names = UserTypeEnum::to_comma_str(&list);
                    model.user_type = Set(type_names);
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
        model.updated_at = Set(time);

        if is_create {
            model.created_at = Set(time);

            let no = dto.no.clone();
            let no = match no {
                Some(str) => str,
                None => {
                    let rand_str = random::alpha_string(RAND_NO_LENGTH);
                    format!("U{}", rand_str)
                }
            };
            model.no = Set(no);

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
        }

        let model = model.save(db).await?;
        let model = model.try_into_model()?;

        handle_ok(model)
    }

    async fn is_column_exist(
        id: i64,
        column: UserColumn,
        value: sea_orm::Value,
        db: &DatabaseConnection,
    ) -> HandleResult<bool> {
        let count = UserEntity::find()
            .select_only()
            .column(UserColumn::Id)
            .filter(column.eq(value))
            .filter(UserColumn::Id.ne(id))
            .count(db)
            .await?;

        handle_ok(count > 0)
    }

    /// 表单选项
    pub fn form_options() -> HandleResult<UserFormOptionVO> {
        let genders = GenderEnum::to_option_list();
        let types = UserTypeEnum::to_option_list();

        let vo = UserFormOptionVO {
            genders: Some(genders),
            types: Some(types),
        };

        handle_ok(vo)
    }

    /// 检查字段值是否唯一
    pub async fn field_unique(
        form: &FieldValueUniqueForm,
        db: &DatabaseConnection,
    ) -> HandleResult<bool> {
        let id = form.skip_id.unwrap_or(0);

        let name = form.field_name.clone().unwrap();
        let column = match name.to_lowercase().as_str() {
            "name" | "username" => UserColumn::Name,
            "email" => UserColumn::Email,
            "mobile" | "phone" => UserColumn::Phone,
            _ => {
                let err = AppError::BadRequest(String::from("无效的字段"));
                return Err(err);
            }
        };

        let value = form.field_value.clone().unwrap();
        let value = sea_orm::Value::from(value);

        let exist = Self::is_column_exist(id, column, value, db).await?;
        handle_ok(exist != true)
    }
}
