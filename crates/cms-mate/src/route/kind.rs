use salvo::oapi::extract::*;
use salvo::prelude::*;
use validator::Validate;

use cms_core::{
    config::AppState,
    domain::{
        AppResult,
        dto::FieldBoolUpdateDTO,
        form::{FieldBoolUpdateForm, FieldValueUniqueForm},
        result_ok,
        vo::PaginateResultVO,
    },
    enums::PlatformEnum,
};

use crate::{
    domain::{
        dto::{KindQueryDTO, KindStoreDTO, KindViewDTO},
        form::KindStoreForm,
        query::KindPaginateQuery,
        vo::{KindFormOptionVO, KindMasterVO, KindQueryOptionVO},
    },
    enums::KindLoadEnum,
    service::KindService,
};

/// 分页列表
///
/// 管理端分页查询
#[endpoint(
    parameters(KindPaginateQuery),
    tags("Mate模块/管理端/类型管理"),
    responses(
        (status_code = 200, description = "success response")
    )
)]
pub async fn manager_paginate(
    depot: &mut Depot,
    query: KindPaginateQuery,
) -> AppResult<PaginateResultVO<KindMasterVO>> {
    let state = depot.obtain::<AppState>().unwrap();
    let mut dto: KindQueryDTO = query.into();
    dto.load_models = Some(vec![KindLoadEnum::Editor]);
    let vo = KindService::paginage(&PlatformEnum::Manager, &dto, state).await?;
    result_ok(vo)
}

/// 创建用户
///
/// 管理端创建用户
#[endpoint(
    tags("Mate模块/管理端/类型管理"),
    responses(
        (status_code = 200, description = "success response")
    )
)]
pub async fn manager_create(depot: &mut Depot, json: JsonBody<KindStoreForm>) -> AppResult<String> {
    let form = json.into_inner();
    form.validate()?;
    let state = depot.obtain::<AppState>().unwrap();
    let dto = form.into();
    KindService::store(&PlatformEnum::Manager, &dto, state).await?;
    result_ok("oK".to_string())
}

/// 更新用户
///
/// 管理端更新用户
#[endpoint(
    tags("Mate模块/管理端/类型管理"),
    responses(
        (status_code = 200, description = "success response")
    )
)]
pub async fn manager_update(
    depot: &mut Depot,
    id: PathParam<i64>,
    json: JsonBody<KindStoreForm>,
) -> AppResult<String> {
    let form = json.into_inner();
    form.validate()?;
    let state = depot.obtain::<AppState>().unwrap();
    let mut dto: KindStoreDTO = form.into();
    dto.id = Some(id.into_inner());
    KindService::store(&PlatformEnum::Manager, &dto, state).await?;
    result_ok("oK".to_string())
}

/// 删除用户
///
/// 管理端删除用户
#[endpoint(
    tags("Mate模块/管理端/类型管理"),
    responses(
        (status_code = 200, description = "success response")
    )
)]
pub async fn manager_delete(depot: &mut Depot, id: PathParam<i64>) -> AppResult<bool> {
    let state = depot.obtain::<AppState>().unwrap();
    let id = id.into_inner();
    KindService::destroy(id, state).await?;
    result_ok(true)
}

/// 表单选项
///
/// 管理端表单选项
#[endpoint(
    tags("Mate模块/管理端/类型管理"),
    responses(
        (status_code = 200, description = "success response")
    )
)]
pub async fn manager_form(depot: &mut Depot) -> AppResult<KindFormOptionVO> {
    let state = depot.obtain::<AppState>().unwrap();
    let vo = KindService::form_options(&PlatformEnum::Manager, &state).await?;
    result_ok(vo)
}

/// 查询选项
///
/// 管理端查询选项
#[endpoint(
    tags("Mate模块/管理端/类型管理"),
    responses(
        (status_code = 200, description = "success response")
    )
)]
pub async fn manager_query(depot: &mut Depot) -> AppResult<KindQueryOptionVO> {
    let state = depot.obtain::<AppState>().unwrap();
    let vo = KindService::query_options(&PlatformEnum::Manager, &state).await?;
    result_ok(vo)
}

/// 唯一性校验
///
/// 管理端字段值唯一性校验
#[endpoint(
    tags("Mate模块/管理端/类型管理"),
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
    let value = KindService::field_unique(&dto, state).await?;
    result_ok(value)
}

/// 更新Bool字段值
///
/// 管理端更新Bool字段值
#[endpoint(
    tags("Mate模块/管理端/类型管理"),
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
    let value = KindService::update_bool_field(&dto, state).await?;
    result_ok(value)
}

/// 查看详情
///
/// 管理端查看详情
#[endpoint(
    tags("Mate模块/管理端/类型管理"),
    responses(
        (status_code = 200, description = "success response")
    )
)]
pub async fn manager_view(depot: &mut Depot, id: PathParam<i64>) -> AppResult<KindMasterVO> {
    let load_models: Vec<KindLoadEnum> = vec![];
    let dto = KindViewDTO {
        id: id.into_inner(),
        load_models: Some(load_models),
        ..Default::default()
    };
    let state = depot.obtain::<AppState>().unwrap();
    let model = KindService::view(&PlatformEnum::Manager, &dto, state).await?;
    result_ok(model)
}
