use salvo::oapi::extract::*;
use salvo::prelude::*;
use validator::Validate;

use cms_core::{
    config::AppState,
    domain::{
        AppResult,
        dto::{FieldBoolUpdateDTO, ModelLogicDeleteDTO},
        form::{FieldBoolUpdateForm, FieldValueUniqueForm},
        result_ok,
        vo::PaginateResultVO,
    },
    enums::PlatformEnum,
    utils::get_current_editor,
};

use crate::{
    domain::{
        dto::{AppQueryDTO, AppStoreDTO, AppViewDTO},
        form::AppStoreForm,
        query::AppPaginateQuery,
        vo::{AppFormOptionVO, AppMasterVO, AppQueryOptionVO},
    },
    enums::AppLoadEnum,
    service::AppService,
};

/// 分页列表
///
/// 管理端分页查询
#[endpoint(
    parameters(AppPaginateQuery),
    tags("Mate模块/管理端/应用管理"),
    responses(
        (status_code = 200, description = "success response")
    )
)]
pub async fn manager_paginate(
    depot: &mut Depot,
    query: AppPaginateQuery,
) -> AppResult<PaginateResultVO<AppMasterVO>> {
    let state = depot.obtain::<AppState>().unwrap();

    let mut dto: AppQueryDTO = query.into();
    dto.load_models = Some(vec![AppLoadEnum::Editor]);
    dto.editor = get_current_editor(depot);

    let vo = AppService::paginage(&PlatformEnum::Manager, &dto, state).await?;
    result_ok(vo)
}

/// 创建应用
///
/// 管理端创建应用
#[endpoint(
    tags("Mate模块/管理端/应用管理"),
    responses(
        (status_code = 200, description = "success response")
    )
)]
pub async fn manager_create(depot: &mut Depot, json: JsonBody<AppStoreForm>) -> AppResult<String> {
    let form = json.into_inner();
    form.validate()?;

    let state = depot.obtain::<AppState>().unwrap();
    let mut dto: AppStoreDTO = form.into();
    dto.editor = get_current_editor(depot);

    AppService::store(&PlatformEnum::Manager, &dto, state).await?;
    result_ok("oK".to_string())
}

/// 更新应用
///
/// 管理端更新应用
#[endpoint(
    tags("Mate模块/管理端/应用管理"),
    responses(
        (status_code = 200, description = "success response")
    )
)]
pub async fn manager_update(
    depot: &mut Depot,
    id: PathParam<i64>,
    json: JsonBody<AppStoreForm>,
) -> AppResult<String> {
    let form = json.into_inner();
    form.validate()?;

    let state = depot.obtain::<AppState>().unwrap();
    let mut dto: AppStoreDTO = form.into();
    dto.id = id.into_inner();
    dto.editor = get_current_editor(depot);

    AppService::store(&PlatformEnum::Manager, &dto, state).await?;
    result_ok("oK".to_string())
}

/// 删除应用
///
/// 管理端删除应用
#[endpoint(
    tags("Mate模块/管理端/应用管理"),
    responses(
        (status_code = 200, description = "success response")
    )
)]
pub async fn manager_delete(depot: &mut Depot, id: PathParam<i64>) -> AppResult<bool> {
    let id = id.into_inner();
    let state = depot.obtain::<AppState>().unwrap();

    let dto = ModelLogicDeleteDTO {
        id,
        editor: get_current_editor(depot),
    };

    AppService::logic_delete(&dto, state).await?;
    result_ok(true)
}

/// 表单选项
///
/// 管理端表单选项
#[endpoint(
    tags("Mate模块/管理端/应用管理"),
    responses(
        (status_code = 200, description = "success response")
    )
)]
pub async fn manager_form(depot: &mut Depot) -> AppResult<AppFormOptionVO> {
    let state = depot.obtain::<AppState>().unwrap();
    let vo = AppService::form_options(&PlatformEnum::Manager, &state).await?;
    result_ok(vo)
}

/// 查询选项
///
/// 管理端查询选项
#[endpoint(
    tags("Mate模块/管理端/应用管理"),
    responses(
        (status_code = 200, description = "success response")
    )
)]
pub async fn manager_query(depot: &mut Depot) -> AppResult<AppQueryOptionVO> {
    let state = depot.obtain::<AppState>().unwrap();
    let vo = AppService::query_options(&PlatformEnum::Manager, &state).await?;
    result_ok(vo)
}

/// 唯一性校验
///
/// 管理端字段值唯一性校验
#[endpoint(
    tags("Mate模块/管理端/应用管理"),
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
    let value = AppService::field_unique(&dto, state).await?;
    result_ok(value)
}

/// 更新Bool字段值
///
/// 管理端更新Bool字段值
#[endpoint(
    tags("Mate模块/管理端/应用管理"),
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
    dto.editor = get_current_editor(depot);

    let state = depot.obtain::<AppState>().unwrap();
    let value = AppService::update_bool_field(&dto, state).await?;
    result_ok(value)
}

/// 查看详情
///
/// 管理端查看详情
#[endpoint(
    tags("Mate模块/管理端/应用管理"),
    responses(
        (status_code = 200, description = "success response")
    )
)]
pub async fn manager_view(depot: &mut Depot, id: PathParam<i64>) -> AppResult<AppMasterVO> {
    let load_models: Vec<AppLoadEnum> = vec![];
    let state = depot.obtain::<AppState>().unwrap();
    let dto = AppViewDTO {
        id: id.into_inner(),
        load_models: Some(load_models),
        editor: get_current_editor(depot),
        ..Default::default()
    };
    let model = AppService::view(&PlatformEnum::Manager, &dto, state).await?;
    result_ok(model)
}
