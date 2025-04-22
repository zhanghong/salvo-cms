use sea_orm::*;

use cms_core::{
    config::AppState,
    domain::{
        HandleResult, dto::JwtClaimsDTO, entity::certificate::Model as CertificateModel, handle_ok,
    },
    enums::PlatformEnum,
    error::AppError,
    service::JwtService,
    utils::{encrypt::encrypt_password, time},
};

use crate::domain::entity::login::{ActiveModel as LoginActiveModel, Model as LoginModel};
use crate::domain::entity::user::{
    ActiveModel as UserActiveModel, Column as UserColumn, Entity as UserEntity, Model as UserModel,
};
use crate::domain::vo::LoginTokenCreateVO;
use crate::domain::{dto::LoginStoreDTO, vo::LoginTokenUpdateVO};

pub struct LoginService {}

impl LoginService {
    pub async fn store(
        platform: &PlatformEnum,
        dto: &LoginStoreDTO,
        state: &AppState,
    ) -> HandleResult<LoginTokenCreateVO> {
        let username: Option<String> = dto.username.clone();
        if username.is_none() {
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

        let username = username.unwrap().to_lowercase().trim().to_string();
        let condition = Condition::any()
            .add(UserColumn::Name.eq(&username))
            .add(UserColumn::Phone.eq(&username))
            .add(UserColumn::Email.eq(&username));
        let user = UserEntity::find().filter(condition).one(&state.db).await?;
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
        println!("md5_password: {}", md5_password);
        if md5_password.ne(&user.password) {
            let err = AppError::BadRequest(String::from("密码错误"));
            return Err(err);
        }

        let login_type = match platform {
            PlatformEnum::Manager => "manager",
            _ => "member",
        };
        let cert: CertificateModel = JwtService::create(user.id, login_type, state)
            .await
            .unwrap();
        let avatar = user.avatar_url();
        let roles: Vec<String> = vec![login_type.to_string()];
        let permissions: Vec<String> = vec![];
        let vo = LoginTokenCreateVO {
            user_id: user.id,
            username: user.name.to_owned(),
            nickname: user.nickname.to_owned(),
            avatar,
            roles,
            permissions,
            access_token: cert.access_token.to_owned(),
            access_expired: time::to_db_time(&cert.access_expired_at),
            refresh_token: cert.refresh_token.to_owned(),
            refresh_expired: time::to_db_time(&cert.refresh_expired_at),
        };

        // 记录登录日志
        let now = time::current_time();
        let login = LoginActiveModel {
            user_id: Set(user.id),
            login_type: Set(login_type.to_owned()),
            client_ip: Set(dto.client_ip.to_owned()),
            user_agent: Set(dto.user_agent.to_owned()),
            created_at: Set(now),
            ..Default::default()
        };
        let login: LoginModel = login.insert(&state.db).await?;

        // 更新用户表里的最后记录信息
        let mut user: UserActiveModel = user.into();
        user.last_login_at = Set(Some(now));
        user.last_login_id = Set(login.id);
        user.updated_at = Set(now);
        user.update(&state.db).await?;

        handle_ok(vo)
    }

    pub async fn update(
        claims: Option<JwtClaimsDTO>,
        state: &AppState,
    ) -> HandleResult<LoginTokenUpdateVO> {
        let cert = JwtService::update_by_claims(claims, state).await?;

        let vo = LoginTokenUpdateVO {
            access_token: cert.access_token.to_owned(),
            access_expired: time::to_db_time(&cert.access_expired_at),
            refresh_token: cert.refresh_token.to_owned(),
            refresh_expired: time::to_db_time(&cert.refresh_expired_at),
        };
        handle_ok(vo)
    }

    pub async fn delete(claims: Option<JwtClaimsDTO>, state: &AppState) -> HandleResult<()> {
        JwtService::delete_by_claims(claims, state).await?;

        handle_ok(())
    }
}
