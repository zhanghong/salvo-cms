use salvo::oapi::extract::*;
use salvo::prelude::*;
use validator::Validate;

use cms_core::{
    config::AppState,
    domain::{result_ok, AppResult},
    enums::PlatformEnum,
};

use crate::{
    domain::{form::UserCreateForm, query::UserPaginateQuery},
    service::UserService,
};

/// 用户列表
///
/// 管理端分页查询
#[endpoint(
    parameters(UserPaginateQuery),
    tags("用户模块/管理端/用户管理"),
    responses(
        (status_code = 200, description = "success response")
    )
)]
pub async fn manager_paginate(query: UserPaginateQuery) -> AppResult<String> {
    println!("query: {:?}", query);
    result_ok("oK".to_string())
}

/// 创建用户
///
/// 管理端创建用户
#[endpoint(
    tags("用户模块/管理端/用户管理"),
    responses(
        (status_code = 200, description = "success response")
    )
)]
pub async fn manager_create(
    depot: &mut Depot,
    json: JsonBody<UserCreateForm>,
) -> AppResult<String> {
    let form = json.into_inner();
    form.validate()?;
    let state = depot.obtain::<AppState>().unwrap();
    println!("form: {:?}", form);
    let dto = form.into();
    UserService::create(PlatformEnum::Manager, &dto, &state.db).await?;
    result_ok("oK".to_string())
}

/// 日志列表
///
/// 管理端查看用户登录日志列表
#[endpoint(
    tags("用户模块/管理端/登录日志"),
    responses(
        (status_code = 200, description = "success response")
    )
)]
pub async fn manager_logs(depot: &mut Depot, form: JsonBody<UserCreateForm>) -> AppResult<String> {
    println!("form: {:?}", form);
    let state = depot.obtain::<AppState>().unwrap();
    let _ = &state.db.ping().await?;
    result_ok("oK".to_string())
}

/// 用户列表
///
/// 管理端分页查询
#[endpoint(
    parameters(UserPaginateQuery),
    tags("用户模块/用户端/用户管理"),
    responses(
        (status_code = 200, description = "success response")
    )
)]
pub async fn open_paginate(query: UserPaginateQuery) -> AppResult<String> {
    println!("query: {:?}", query);
    result_ok("oK".to_string())
}

/// 创建用户
///
/// 管理端创建用户
#[endpoint(
    tags("用户模块/用户端/用户管理"),
    responses(
        (status_code = 200, description = "success response")
    )
)]
pub async fn open_create(depot: &mut Depot, form: FormBody<UserCreateForm>) -> AppResult<String> {
    println!("form: {:?}", form);
    let state = depot.obtain::<AppState>().unwrap();
    let _ = &state.db.ping().await?;
    result_ok("oK".to_string())
}
