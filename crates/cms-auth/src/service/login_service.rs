use sea_orm::*;

use cms_core::{
    config::AppState,
    domain::{
        HandleResult, dto::JwtClaimsDTO, entity::certificate::Model as CertificateModel, handle_ok,
    },
    enums::PlatformEnum,
    error::AppError,
    service::JwtService,
    utils::{encrypt_utils::encrypt_password, time_utils},
};

use crate::domain::entity::login::{ActiveModel as LoginActiveModel, Model as LoginModel};
use crate::domain::entity::user::{
    ActiveModel as UserActiveModel, Column as UserColumn, Entity as UserEntity,
};
use crate::domain::vo::TokenCreateVO;
use crate::domain::{dto::LoginStoreDTO, vo::TokenUpdateVO};

pub struct LoginService {}

impl LoginService {
    pub async fn store(
        platform: &PlatformEnum,
        dto: &LoginStoreDTO,
        state: &AppState,
    ) -> HandleResult<TokenCreateVO> {
        let username = match dto.username.as_ref() {
            Some(username) => username.trim().to_lowercase(),
            None => return Err(AppError::BadRequest(String::from("登录名不能为空"))),
        };

        let password = match dto.password.as_ref() {
            Some(password) => password.trim(),
            None => return Err(AppError::BadRequest(String::from("密码不能为空"))),
        };

        let condition = Condition::any()
            .add(UserColumn::Name.eq(&username))
            .add(UserColumn::Phone.eq(&username))
            .add(UserColumn::Email.eq(&username));

        let user = match UserEntity::find().filter(condition).one(&state.db).await? {
            Some(user) => user,
            None => return Err(AppError::BadRequest(String::from("用户不存在"))),
        };

        if user.is_enabled == false {
            return Err(AppError::BadRequest(String::from("用户已被禁用")));
        }

        let md5_password = encrypt_password(user.salt.as_str(), password);
        println!("md5_password: {}", md5_password);
        if md5_password.ne(&user.password) {
            return Err(AppError::BadRequest(String::from("密码错误")));
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

        let vo = TokenCreateVO {
            user_id: user.id,
            username: user.name.to_owned(),
            nickname: user.nickname.to_owned(),
            avatar,
            roles,
            permissions,
            access_token: cert.access_token.to_owned(),
            access_expired: time_utils::to_db_time(&cert.access_expired_at),
            refresh_token: cert.refresh_token.to_owned(),
            refresh_expired: time_utils::to_db_time(&cert.refresh_expired_at),
        };

        // 记录登录日志
        let now = time_utils::current_time();
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
        user.last_login_id = Set(Some(login.id));
        user.updated_at = Set(now);
        user.update(&state.db).await?;

        handle_ok(vo)
    }

    pub async fn update(
        claims: Option<JwtClaimsDTO>,
        state: &AppState,
    ) -> HandleResult<TokenUpdateVO> {
        let cert = JwtService::update_by_claims(claims, state).await?;

        let vo = TokenUpdateVO {
            access_token: cert.access_token.to_owned(),
            access_expired: time_utils::to_db_time(&cert.access_expired_at),
            refresh_token: cert.refresh_token.to_owned(),
            refresh_expired: time_utils::to_db_time(&cert.refresh_expired_at),
        };
        handle_ok(vo)
    }

    pub async fn delete(claims: Option<JwtClaimsDTO>, state: &AppState) -> HandleResult<()> {
        JwtService::delete_by_claims(claims, state).await?;

        handle_ok(())
    }
}
