use salvo::oapi::extract::*;
use salvo::prelude::*;
use validator::Validate;

use cms_core::{
    config::AppState,
    domain::{
        AppResult,
        dto::{FieldBoolUpdateDTO, ModelLogicDeleteDTO, ModelViewDTO},
        form::{FieldBoolUpdateForm, FieldValueUniqueForm},
        result_ok,
        vo::PaginateResultVO,
    },
    enums::PlatformEnum,
    utils::get_current_editor,
};

use crate::{
    domain::{
        dto::{AppQueryDTO, AppStoreDTO},
        form::AppStoreForm,
        query::AppPaginateQuery,
        vo::{AppFormOptionVO, AppMasterVO, AppQueryOptionVO},
    },
    enums::AppLoadEnum,
    service::AppService,
};

/// Paginate List
///
/// paginate query list
#[endpoint(
    operation_id = "mate_app_manager_paginate",
    security(["bearer" = ["bearer"]]),
    tags("Mate/Manager/App")
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

/// Create App
///
/// Create app
#[endpoint(
    operation_id = "mate_app_manager_create",
    security(["bearer" = ["bearer"]]),
    tags("Mate/Manager/App")
)]
pub async fn manager_create(depot: &mut Depot, json: JsonBody<AppStoreForm>) -> AppResult<bool> {
    let form = json.into_inner();
    form.validate()?;

    let state = depot.obtain::<AppState>().unwrap();
    let mut dto: AppStoreDTO = form.into();
    dto.editor = get_current_editor(depot);

    AppService::store(&PlatformEnum::Manager, &dto, state).await?;
    result_ok(true)
}

/// Update App
///
/// update app
#[endpoint(
    operation_id = "mate_app_manager_update",
    security(["bearer" = ["bearer"]]),
    tags("Mate/Manager/App")
)]
pub async fn manager_update(
    depot: &mut Depot,
    id: PathParam<i64>,
    json: JsonBody<AppStoreForm>,
) -> AppResult<bool> {
    let form = json.into_inner();
    form.validate()?;

    let state = depot.obtain::<AppState>().unwrap();
    let mut dto: AppStoreDTO = form.into();
    dto.id = id.into_inner();
    dto.editor = get_current_editor(depot);

    AppService::store(&PlatformEnum::Manager, &dto, state).await?;
    result_ok(true)
}

/// Delete App
///
/// Delete app
#[endpoint(
    operation_id = "mate_app_manager_delete",
    security(["bearer" = ["bearer"]]),
    tags("Mate/Manager/App")
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

/// Form Options
///
/// store form options
#[endpoint(
    operation_id = "mate_app_manager_form",
    security(["bearer" = ["bearer"]]),
    tags("Mate/Manager/App")
)]
pub async fn manager_form(depot: &mut Depot) -> AppResult<AppFormOptionVO> {
    let state = depot.obtain::<AppState>().unwrap();
    let vo = AppService::form_options(&PlatformEnum::Manager, &state).await?;
    result_ok(vo)
}

/// Query Options
///
/// Paginate query options
#[endpoint(
    operation_id = "mate_app_manager_query",
    security(["bearer" = ["bearer"]]),
    tags("Mate/Manager/App")
)]
pub async fn manager_query(depot: &mut Depot) -> AppResult<AppQueryOptionVO> {
    let state = depot.obtain::<AppState>().unwrap();
    let vo = AppService::query_options(&PlatformEnum::Manager, &state).await?;
    result_ok(vo)
}

/// Field Unique
///
/// Field unique check
#[endpoint(
    operation_id = "mate_app_check_field_unique",
    security(["bearer" = ["bearer"]]),
    tags("Mate/Manager/App")
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

/// Update Bool Field
///
/// Update Bool Field
#[endpoint(
    operation_id = "mate_app_update_bool_field",
    security(["bearer" = ["bearer"]]),
    tags("Mate/Manager/App")
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

/// View App
///
/// View app
#[endpoint(
    operation_id = "mate_app_manager_view",
    security(["bearer" = ["bearer"]]),
    tags("Mate/Manager/App")
)]
pub async fn manager_view(depot: &mut Depot, id: PathParam<i64>) -> AppResult<AppMasterVO> {
    let load_models: Vec<AppLoadEnum> = vec![];
    let state = depot.obtain::<AppState>().unwrap();
    let dto = ModelViewDTO {
        id: id.into_inner(),
        load_models: Some(load_models),
        editor: get_current_editor(depot),
        ..Default::default()
    };
    let model = AppService::view(&PlatformEnum::Manager, &dto, state).await?;
    result_ok(model)
}
