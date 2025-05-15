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
        dto::{KindQueryDTO, KindStoreDTO},
        form::KindStoreForm,
        query::KindPaginateQuery,
        vo::{KindFormOptionVO, KindMasterVO, KindQueryOptionVO},
    },
    enums::KindLoadEnum,
    service::KindService,
};

/// Paginate
///
/// Paginate Kind
#[endpoint(
    operation_id = "mate_kind_manager_paginate",
    security(["bearer" = ["bearer"]]),
    tags("Mate/Manager/Kind")
)]
pub async fn manager_paginate(
    depot: &mut Depot,
    query: KindPaginateQuery,
) -> AppResult<PaginateResultVO<KindMasterVO>> {
    let state = depot.obtain::<AppState>().unwrap();

    let mut dto: KindQueryDTO = query.into();
    dto.load_models = Some(vec![KindLoadEnum::Editor]);
    dto.editor = get_current_editor(depot);

    let vo = KindService::paginage(&PlatformEnum::Manager, &dto, state).await?;
    result_ok(vo)
}

/// Create kind
///
/// Create kind
#[endpoint(
    operation_id = "mate_kind_manager_create",
    security(["bearer" = ["bearer"]]),
    tags("Mate/Manager/Kind"),
    status_codes(200, 400, 401)
)]
pub async fn manager_create(depot: &mut Depot, json: JsonBody<KindStoreForm>) -> AppResult<bool> {
    let form = json.into_inner();
    form.validate()?;

    let state = depot.obtain::<AppState>().unwrap();
    let mut dto: KindStoreDTO = form.into();
    dto.editor = get_current_editor(depot);

    KindService::store(&PlatformEnum::Manager, &dto, state).await?;
    result_ok(true)
}

/// Update kind
///
/// Update kind
#[endpoint(
    operation_id = "mate_kind_manager_update",
    security(["bearer" = ["bearer"]]),
    tags("Mate/Manager/Kind"),
    status_codes(200, 400, 401, 404)
)]
pub async fn manager_update(
    depot: &mut Depot,
    id: PathParam<i64>,
    json: JsonBody<KindStoreForm>,
) -> AppResult<bool> {
    let form = json.into_inner();
    form.validate()?;

    let state = depot.obtain::<AppState>().unwrap();
    let mut dto: KindStoreDTO = form.into();
    dto.id = id.into_inner();
    dto.editor = get_current_editor(depot);

    KindService::store(&PlatformEnum::Manager, &dto, state).await?;
    result_ok(true)
}

/// Delete kind
///
/// Delete kind by id
#[endpoint(
    operation_id = "mate_kind_manager_delete",
    security(["bearer" = ["bearer"]]),
    tags("Mate/Manager/Kind")
)]
pub async fn manager_delete(depot: &mut Depot, id: PathParam<i64>) -> AppResult<bool> {
    let id = id.into_inner();
    let state = depot.obtain::<AppState>().unwrap();

    let dto = ModelLogicDeleteDTO {
        id,
        editor: get_current_editor(depot),
    };

    KindService::logic_delete(&dto, state).await?;
    result_ok(true)
}

/// Form Options
///
/// Store form options
#[endpoint(
    operation_id = "mate_kind_manager_form",
    security(["bearer" = ["bearer"]]),
    tags("Mate/Manager/Kind")
)]
pub async fn manager_form(depot: &mut Depot) -> AppResult<KindFormOptionVO> {
    let state = depot.obtain::<AppState>().unwrap();
    let vo = KindService::form_options(&PlatformEnum::Manager, &state).await?;
    result_ok(vo)
}

/// Query Options
///
/// Paginate Query Options
#[endpoint(
    operation_id = "mate_kind_manager_query",
    security(["bearer" = ["bearer"]]),
    tags("Mate/Manager/Kind")
)]
pub async fn manager_query(depot: &mut Depot) -> AppResult<KindQueryOptionVO> {
    let state = depot.obtain::<AppState>().unwrap();
    let vo = KindService::query_options(&PlatformEnum::Manager, &state).await?;
    result_ok(vo)
}

/// Field Unique
///
/// Field unique check
#[endpoint(
    operation_id = "mate_app_check_field_unique",
    security(["bearer" = ["bearer"]]),
    tags("Mate/Manager/Kind")
)]
pub async fn check_field_unique(
    depot: &mut Depot,
    json: JsonBody<FieldValueUniqueForm>,
) -> AppResult<bool> {
    let form = json.into_inner();
    form.validate()?;
    let dto = form.into();
    let state = depot.obtain::<AppState>().unwrap();
    let value = KindService::field_unique(&dto, state).await?;
    result_ok(value)
}

/// Update Bool Field
///
/// Update Bool Field
#[endpoint(
    operation_id = "mate_app_update_bool_field",
    security(["bearer" = ["bearer"]]),
    tags("Mate/Manager/Kind")
)]
pub async fn update_bool_field(
    depot: &mut Depot,
    id: PathParam<i64>,
    json: JsonBody<FieldBoolUpdateForm>,
) -> AppResult<bool> {
    let form = json.into_inner();
    form.validate()?;

    let mut dto: FieldBoolUpdateDTO = form.into();
    dto.id = id.into_inner();
    dto.editor = get_current_editor(depot);

    let state = depot.obtain::<AppState>().unwrap();
    let value = KindService::update_bool_field(&dto, state).await?;
    result_ok(value)
}

/// View kind
///
/// View kind by id
#[endpoint(
    operation_id = "mate_kind_manager_view",
    security(["bearer" = ["bearer"]]),
    tags("Mate/Manager/Kind")
)]
pub async fn manager_view(depot: &mut Depot, id: PathParam<i64>) -> AppResult<KindMasterVO> {
    let load_models: Vec<KindLoadEnum> = vec![];
    let state = depot.obtain::<AppState>().unwrap();

    let dto = ModelViewDTO {
        id: id.into_inner(),
        load_models: Some(load_models),
        editor: get_current_editor(depot),
        ..Default::default()
    };
    let model = KindService::view(&PlatformEnum::Manager, &dto, state).await?;
    result_ok(model)
}
