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
        dto::{ItemQueryDTO, ItemStoreDTO},
        form::ItemStoreForm,
        query::ItemPaginateQuery,
        vo::{ItemFormOptionVO, ItemMasterVO, ItemQueryOptionVO},
    },
    enums::ItemLoadEnum,
    service::ItemService,
};

/// 分页列表
///
/// 管理端分页查询
#[endpoint(
    parameters(ItemPaginateQuery),
    tags("Mate模块/管理端/Item管理"),
    responses(
        (status_code = 200, description = "success response")
    )
)]
pub async fn manager_paginate(
    depot: &mut Depot,
    query: ItemPaginateQuery,
) -> AppResult<PaginateResultVO<ItemMasterVO>> {
    let state = depot.obtain::<AppState>().unwrap();

    let mut dto: ItemQueryDTO = query.into();
    dto.load_models = Some(vec![ItemLoadEnum::Editor]);
    dto.editor = get_current_editor(depot);

    let vo = ItemService::paginage(&PlatformEnum::Manager, &dto, state).await?;
    result_ok(vo)
}

/// 创建内容
///
/// 管理端创建内容
#[endpoint(
    tags("Mate模块/管理端/Item管理"),
    responses(
        (status_code = 200, description = "success response")
    )
)]
pub async fn manager_create(depot: &mut Depot, json: JsonBody<ItemStoreForm>) -> AppResult<bool> {
    let form = json.into_inner();
    form.validate()?;

    let state = depot.obtain::<AppState>().unwrap();
    let mut dto: ItemStoreDTO = form.into();
    dto.editor = get_current_editor(depot);

    ItemService::store(&PlatformEnum::Manager, &dto, state).await?;
    result_ok(true)
}

/// 更新内容
///
/// 管理端更新内容
#[endpoint(
    tags("Mate模块/管理端/Item管理"),
    responses(
        (status_code = 200, description = "success response")
    )
)]
pub async fn manager_update(
    depot: &mut Depot,
    id: PathParam<i64>,
    json: JsonBody<ItemStoreForm>,
) -> AppResult<bool> {
    let form = json.into_inner();
    form.validate()?;

    let state = depot.obtain::<AppState>().unwrap();
    let mut dto: ItemStoreDTO = form.into();
    dto.id = id.into_inner();
    dto.editor = get_current_editor(depot);

    ItemService::store(&PlatformEnum::Manager, &dto, state).await?;
    result_ok(true)
}

/// 删除内容
///
/// 管理端删除内容
#[endpoint(
    tags("Mate模块/管理端/Item管理"),
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

    ItemService::logic_delete(&dto, state).await?;
    result_ok(true)
}

/// 表单选项
///
/// 管理端表单选项
#[endpoint(
    tags("Mate模块/管理端/Item管理"),
    responses(
        (status_code = 200, description = "success response")
    )
)]
pub async fn manager_form(depot: &mut Depot) -> AppResult<ItemFormOptionVO> {
    let state = depot.obtain::<AppState>().unwrap();
    let vo = ItemService::form_options(&PlatformEnum::Manager, &state).await?;
    result_ok(vo)
}

/// 查询选项
///
/// 管理端查询选项
#[endpoint(
    tags("Mate模块/管理端/Item管理"),
    responses(
        (status_code = 200, description = "success response")
    )
)]
pub async fn manager_query(depot: &mut Depot) -> AppResult<ItemQueryOptionVO> {
    let state = depot.obtain::<AppState>().unwrap();
    let vo = ItemService::query_options(&PlatformEnum::Manager, &state).await?;
    result_ok(vo)
}

/// 唯一性校验
///
/// 管理端字段值唯一性校验
#[endpoint(
    tags("Mate模块/管理端/Item管理"),
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
    let value = ItemService::field_unique(&dto, state).await?;
    result_ok(value)
}

/// 更新Bool字段值
///
/// 管理端更新Bool字段值
#[endpoint(
    tags("Mate模块/管理端/Item管理"),
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

    let mut dto: FieldBoolUpdateDTO = form.into();
    dto.id = id.into_inner();
    dto.editor = get_current_editor(depot);

    let state = depot.obtain::<AppState>().unwrap();
    let value = ItemService::update_bool_field(&dto, state).await?;
    result_ok(value)
}

/// 查看详情
///
/// 管理端查看详情
#[endpoint(
    tags("Mate模块/管理端/Item管理"),
    responses(
        (status_code = 200, description = "success response")
    )
)]
pub async fn manager_view(depot: &mut Depot, id: PathParam<i64>) -> AppResult<ItemMasterVO> {
    let load_models: Vec<ItemLoadEnum> = vec![];
    let dto = ModelViewDTO {
        id: id.into_inner(),
        load_models: Some(load_models),
        editor: get_current_editor(depot),
        ..Default::default()
    };
    let state = depot.obtain::<AppState>().unwrap();
    let model = ItemService::view(&PlatformEnum::Manager, &dto, state).await?;
    result_ok(model)
}
