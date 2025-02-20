use salvo::oapi::extract::*;
use salvo::prelude::*;
use validator::Validate;

use cms_core::{
    config::AppState,
    domain::{
        dto::FieldBoolUpdateDTO,
        form::{FieldBoolUpdateForm, FieldValueUniqueForm},
        result_ok,
        vo::PaginateResultVO,
        AppResult,
    },
    enums::PlatformEnum,
};

use crate::{
    domain::{
        dto::{UserQueryDTO, UserStoreDTO, UserUpdatePasswordDTO, UserViewDTO},
        form::{UserCreateForm, UserUpdateForm, UserUpdatePasswordForm},
        query::UserPaginateQuery,
        vo::{UserFormOptionVO, UserItemVO},
    },
    enums::UserLoadEnum,
    service::UserService,
};

/// 分页列表
///
/// 管理端分页查询
#[endpoint(
    parameters(UserPaginateQuery),
    tags("用户模块/管理端/用户管理"),
    responses(
        (status_code = 200, description = "success response")
    )
)]
pub async fn manager_paginate(
    depot: &mut Depot,
    query: UserPaginateQuery,
) -> AppResult<PaginateResultVO<UserItemVO>> {
    let state = depot.obtain::<AppState>().unwrap();
    let mut dto: UserQueryDTO = query.into();
    dto.load_models = Some(vec![UserLoadEnum::Editor]);
    let vo = UserService::paginage(&PlatformEnum::Manager, &dto, state).await?;
    result_ok(vo)
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
    let dto = form.into();
    UserService::store(&PlatformEnum::Manager, &dto, state).await?;
    result_ok("oK".to_string())
}

/// 更新用户
///
/// 管理端更新用户
#[endpoint(
    tags("用户模块/管理端/用户管理"),
    responses(
        (status_code = 200, description = "success response")
    )
)]
pub async fn manager_update(
    depot: &mut Depot,
    id: PathParam<i64>,
    json: JsonBody<UserUpdateForm>,
) -> AppResult<String> {
    let form = json.into_inner();
    form.validate()?;
    let state = depot.obtain::<AppState>().unwrap();
    let mut dto: UserStoreDTO = form.into();
    dto.id = Some(id.into_inner());
    UserService::store(&PlatformEnum::Manager, &dto, state).await?;
    result_ok("oK".to_string())
}

/// 删除用户
///
/// 管理端删除用户
#[endpoint(
    tags("用户模块/管理端/用户管理"),
    responses(
        (status_code = 200, description = "success response")
    )
)]
pub async fn manager_delete(depot: &mut Depot, id: PathParam<i64>) -> AppResult<bool> {
    let state = depot.obtain::<AppState>().unwrap();
    let id = id.into_inner();
    UserService::destroy(id, state).await?;
    result_ok(true)
}

/// 表单选项
///
/// 管理端表单选项
#[endpoint(
    tags("用户模块/管理端/用户管理"),
    responses(
        (status_code = 200, description = "success response")
    )
)]
pub fn manager_form() -> AppResult<UserFormOptionVO> {
    let vo = UserService::form_options()?;
    result_ok(vo)
}

/// 唯一性校验
///
/// 管理端字段值唯一性校验
#[endpoint(
    tags("用户模块/管理端/用户管理"),
    responses(
        (status_code = 200, description = "success response")
    )
)]
pub async fn check_field_unique(
    depot: &mut Depot,
    json: JsonBody<FieldValueUniqueForm>,
) -> AppResult<bool> {
    let form = json.into_inner();
    form.validate()?;
    let dto = form.into();
    let state = depot.obtain::<AppState>().unwrap();
    let value = UserService::field_unique(&dto, state).await?;
    result_ok(value)
}

/// 更新Bool字段值
///
/// 管理端更新Bool字段值
#[endpoint(
    tags("用户模块/管理端/用户管理"),
    responses(
        (status_code = 200, description = "success response")
    )
)]
pub async fn update_bool_field(
    depot: &mut Depot,
    id: PathParam<i64>,
    json: JsonBody<FieldBoolUpdateForm>,
) -> AppResult<bool> {
    let form = json.into_inner();
    form.validate()?;
    // 因为设置 id 所以必须指定 dto 类型
    let mut dto: FieldBoolUpdateDTO = form.into();
    dto.id = id.into_inner();
    let state = depot.obtain::<AppState>().unwrap();
    let value = UserService::update_bool_field(&dto, state).await?;
    result_ok(value)
}

/// 修改密码
///
/// 管理端修改密码
#[endpoint(
    tags("用户模块/管理端/用户管理"),
    responses(
        (status_code = 200, description = "success response")
    )
)]
pub async fn manager_update_password(
    depot: &mut Depot,
    id: PathParam<i64>,
    json: JsonBody<UserUpdatePasswordForm>,
) -> AppResult<bool> {
    let form = json.into_inner();
    form.validate()?;
    let mut dto: UserUpdatePasswordDTO = form.into();
    dto.id = id.into_inner();
    let state = depot.obtain::<AppState>().unwrap();
    let value = UserService::update_password(&dto, state).await?;
    result_ok(value)
}

/// 查看详情
///
/// 管理端查看用户详情
#[endpoint(
    tags("用户模块/管理端/用户管理"),
    responses(
        (status_code = 200, description = "success response")
    )
)]
pub async fn manager_view(depot: &mut Depot, id: PathParam<i64>) -> AppResult<UserItemVO> {
    let load_models: Vec<UserLoadEnum> = vec![UserLoadEnum::Editor, UserLoadEnum::Detail];
    let dto = UserViewDTO {
        id: id.into_inner(),
        load_models: Some(load_models),
        ..Default::default()
    };
    let state = depot.obtain::<AppState>().unwrap();
    let model = UserService::view(&dto, state).await?;
    result_ok(model)
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
