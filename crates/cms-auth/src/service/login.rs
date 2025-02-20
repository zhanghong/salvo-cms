use sea_orm::*;

use cms_core::{
    domain::{handle_ok, HandleResult},
    enums::PlatformEnum,
    error::AppError,
    service::JwtService,
    utils::{encrypt::encrypt_password, time},
};

use crate::domain::dto::LoginStoreDTO;
use crate::domain::entity::login::{ActiveModel as LoginActiveModel, Model as LoginModel};
use crate::domain::entity::user::{
    ActiveModel as UserActiveModel, Column as UserColumn, Entity as UserEntity, Model as UserModel,
};
use crate::domain::vo::LoginTokenCreateVO;

pub struct LoginService {}

impl LoginService {
    pub async fn store(
        platform: &PlatformEnum,
        dto: &LoginStoreDTO,
        db: &DatabaseConnection,
    ) -> HandleResult<LoginTokenCreateVO> {
        let name = dto.name.clone();
        if name.is_none() {
            let err = AppError::BadRequest(String::from("登录名不能为空"));
            return Err(err);
        }

        // TODO: 验证密码
        let password = dto.password.clone();
        if password.is_none() {
            let err = AppError::BadRequest(String::from("密码不能为空"));
            return Err(err);
        }
        let password = password.unwrap();

        let name = name.unwrap().to_lowercase().trim().to_string();
        let condition = Condition::any()
            .add(UserColumn::Name.eq(&name))
            .add(UserColumn::Phone.eq(&name))
            .add(UserColumn::Email.eq(&name));
        let user = UserEntity::find().filter(condition).one(db).await?;
        if user.is_none() {
            let err = AppError::BadRequest(String::from("用户不存在"));
            return Err(err);
        }
        let user: UserModel = user.unwrap();
        if user.is_enabled == false {
            let err = AppError::BadRequest(String::from("用户已被禁用"));
            return Err(err);
        }

        let md5_password = encrypt_password(user.salt.as_str(), password.as_str());
        if md5_password.ne(&user.password) {
            let err = AppError::BadRequest(String::from("密码错误"));
            return Err(err);
        }

        let login_type = match platform {
            PlatformEnum::Manager => "manager",
            _ => "member",
        };
        let token = JwtService::user_login(user.id, login_type).unwrap();
        let avatar = user.avatar_url();
        let roles: Vec<String> = vec![login_type.to_string()];
        let permissions: Vec<String> = vec![];
        let vo = LoginTokenCreateVO {
            user_id: user.id,
            username: user.name.to_owned(),
            nickname: user.nickname.to_owned(),
            avatar: avatar,
            roles: roles,
            permissions: permissions,
            access_token: token.access_token.to_owned(),
            access_expired: token.access_expired,
            refresh_token: token.refresh_token.to_owned(),
            refresh_expired: token.refresh_expired,
        };

        // 记录登录日志
        let now = time::current_time();
        let login = LoginActiveModel {
            user_id: Set(user.id),
            client_ip: Set(dto.client_ip.to_owned()),
            user_agent: Set(dto.user_agent.to_owned()),
            created_at: Set(now),
            ..Default::default()
        };
        let login: LoginModel = login.insert(db).await?;

        // 更新用户表里的最后记录信息
        let mut user: UserActiveModel = user.into();
        user.last_login_at = Set(Some(now));
        user.last_login_id = Set(login.id);
        user.updated_at = Set(now);
        user.update(db).await?;

        handle_ok(vo)
    }
}
