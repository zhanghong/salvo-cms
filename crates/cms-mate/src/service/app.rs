use sea_orm::*;
use std::collections::HashMap;

use cms_core::config::AppState;
use cms_core::domain::{
    HandleResult, SelectOptionItem,
    dto::{FieldBoolUpdateDTO, FieldValueUniqueDTO, ModelLogicDeleteDTO},
    handle_ok,
    vo::PaginateResultVO,
};
use cms_core::enums::{EnableEnum, PlatformEnum};
use cms_core::error::AppError;
use cms_core::service::EditorService;
use cms_core::utils::time;

use crate::domain::dto::{AppQueryDTO, AppStoreDTO, AppViewDTO};
use crate::domain::entity::app::{
    ActiveModel as AppActiveModel, Column as AppColumn, Entity as AppEntity, Model as AppModel,
};
use crate::domain::vo::{AppFormOptionVO, AppLoadVO, AppMasterVO, AppQueryOptionVO};
use crate::enums::AppLoadEnum;

pub struct AppService {}

impl AppService {
    /// 创建/更新
    pub async fn store(
        _platform: &PlatformEnum,
        dto: &AppStoreDTO,
        state: &AppState,
    ) -> HandleResult<AppModel> {
        let id: i64 = dto.id;
        let is_create = if id > 0 { false } else { true };

        let mut current_version_no = 0;
        let mut model = if is_create {
            AppActiveModel {
                ..Default::default()
            }
        } else {
            let model = Self::fetch_by_id(id, state).await?;
            current_version_no = model.version_no;
            model.into()
        };

        // 检查版本号
        if let Some(version_no) = dto.version_no.clone() {
            if !is_create && version_no.ne(&current_version_no) {
                let err = AppError::BadRequest(String::from("版本号错误"));
                return Err(err);
            }
        }
        model.version_no = Set(current_version_no + 1);

        let db = &state.db;

        let filter_extends = HashMap::<String, String>::new();
        if let Some(name) = dto.name.clone() {
            let is_exists = Self::is_column_exist(
                id,
                AppColumn::Name,
                sea_orm::Value::from(name.to_owned()),
                &filter_extends,
                db,
            )
            .await?;
            if is_exists {
                let err = AppError::BadRequest(String::from("名称已存在"));
                return Err(err);
            }
            model.name = Set(name);
        }

        if let Some(title) = dto.title.clone() {
            let is_exists = Self::is_column_exist(
                id,
                AppColumn::Title,
                sea_orm::Value::from(title.to_owned()),
                &filter_extends,
                db,
            )
            .await?;
            if is_exists {
                let err = AppError::BadRequest(String::from("标题已存在"));
                return Err(err);
            }
            model.title = Set(title);
        }

        if let Some(description) = dto.description.clone() {
            model.description = Set(description);
        }

        if let Some(icon) = dto.icon.clone() {
            model.icon = Set(icon);
        }

        if let Some(sort) = dto.sort.clone() {
            model.sort = Set(sort);
        }

        if let Some(is_enabled) = dto.is_enabled.clone() {
            model.is_enabled = Set(is_enabled);
        }

        let time = time::current_time();
        model.updated_at = Set(time);

        if is_create {
            model.created_at = Set(time);
        }

        model.editor_type = Set(dto.editor_type.as_value());
        model.editor_id = Set(dto.editor_id);

        let txn = db.begin().await?;

        let model = model.save(&txn).await?;
        let model = model.try_into_model()?;

        // 提交事务
        txn.commit().await?;

        handle_ok(model)
    }

    /// 检查字段值是否唯一（查询）
    async fn is_column_exist(
        id: i64,
        column: AppColumn,
        value: sea_orm::Value,
        _extends: &HashMap<String, String>,
        db: &DatabaseConnection,
    ) -> HandleResult<bool> {
        let count = Self::scope_active_query()
            .select_only()
            .column(AppColumn::Id)
            .filter(column.eq(value))
            .filter(AppColumn::Id.ne(id))
            .count(db)
            .await?;

        handle_ok(count > 0)
    }

    /// 检查字段值是否唯一
    pub async fn field_unique(dto: &FieldValueUniqueDTO, state: &AppState) -> HandleResult<bool> {
        let id = dto.skip_id;
        let db = &state.db;

        let field_name = dto.field_name.to_owned();
        let column = match field_name.to_lowercase().as_str() {
            "name" => AppColumn::Name,
            "title" => AppColumn::Title,
            _ => {
                let err = AppError::BadRequest(String::from("无效的字段"));
                return Err(err);
            }
        };

        let field_value = dto.field_value.to_owned();
        let value = sea_orm::Value::from(field_value);

        let filter_extends = dto
            .extends
            .clone()
            .unwrap_or(HashMap::<String, String>::new());
        let exist = Self::is_column_exist(id, column, value, &filter_extends, db).await?;
        handle_ok(exist != true)
    }

    /// 查询选项
    pub async fn query_options(
        platform: &PlatformEnum,
        _state: &AppState,
    ) -> HandleResult<AppQueryOptionVO> {
        let mut vo = AppQueryOptionVO { enables: None };

        if *platform == PlatformEnum::Manager {
            let enables = EnableEnum::to_option_list();
            vo.enables = Some(enables);
        }

        handle_ok(vo)
    }

    /// 表单选项
    pub async fn form_options(
        platform: &PlatformEnum,
        _state: &AppState,
    ) -> HandleResult<AppFormOptionVO> {
        let mut vo = AppFormOptionVO { enables: None };

        if *platform == PlatformEnum::Manager {
            let enables = EnableEnum::to_option_list();
            vo.enables = Some(enables);
        }

        handle_ok(vo)
    }

    /// 修改布尔值字段
    pub async fn update_bool_field(
        dto: &FieldBoolUpdateDTO,
        state: &AppState,
    ) -> HandleResult<bool> {
        let id = dto.id;
        if id < 1 {
            let err = AppError::BadRequest(String::from("参数ID错误"));
            return Err(err);
        }
        let db = &state.db;

        let model = Self::fetch_by_id(id, state).await?;
        let mut model: AppActiveModel = model.into();

        let field_name = dto.field_name.to_owned();
        let bool_value = dto.field_value;
        match field_name.to_lowercase().as_str() {
            "is_enabled" | "enabled" => {
                model.is_enabled = Set(bool_value);
            }
            _ => {
                let err = AppError::BadRequest(String::from("更新字段错误"));
                return Err(err);
            }
        };

        let now = time::current_time();
        model.updated_at = Set(now);
        model.editor_type = Set(dto.editor_type.as_value());
        model.editor_id = Set(dto.editor_id);

        let _ = model.save(db).await?;

        handle_ok(true)
    }

    /// 查看
    pub async fn view(
        platform: &PlatformEnum,
        dto: &AppViewDTO,
        state: &AppState,
    ) -> HandleResult<AppMasterVO> {
        let id = dto.id;
        if id < 1 {
            let err = AppError::BadRequest(String::from("参数ID错误"));
            return Err(err);
        }

        let model = Self::fetch_by_id(id, state).await?;
        if *platform == PlatformEnum::Open {
            if !model.is_enabled {
                let err = AppError::NotFound(String::from("访问记录不存在"));
                return Err(err);
            }
        }

        let mut vo: AppMasterVO = model.into();
        if let Some(load_models) = dto.load_models.clone() {
            for enums in load_models {
                match enums {
                    AppLoadEnum::Editor => {
                        vo.editor = EditorService::load_by_id(vo.editor_id.clone(), state).await?;
                    }
                    _ => {}
                }
            }
        }

        handle_ok(vo)
    }

    /// 分页查询
    pub async fn paginage(
        platform: &PlatformEnum,
        dto: &AppQueryDTO,
        state: &AppState,
    ) -> HandleResult<PaginateResultVO<AppMasterVO>> {
        let db = &state.db;
        let mut page = dto.page.unwrap_or(1);
        if page < 1 {
            page = 1;
        }

        let mut page_size = dto.page_size.unwrap_or(20);
        if page_size < 1 {
            page_size = 20;
        } else if page_size > 50 {
            page_size = 50;
        }
        let query = Self::query_builder(platform, dto).await?;
        let paginator = query.paginate(db, page_size);
        let total = paginator.num_items_and_pages().await?;
        let models = paginator.fetch_page(page - 1).await?;
        let len = models.len();
        let mut list: Vec<AppMasterVO> = Vec::with_capacity(len);
        let mut editor_ids: Vec<i64> = Vec::with_capacity(len);
        for model in models.iter() {
            editor_ids.push(model.editor_id);
            let vo: AppMasterVO = model.into();
            list.push(vo);
        }

        if let Some(load_models) = dto.load_models.clone() {
            for enums in load_models {
                match enums {
                    AppLoadEnum::Editor => {
                        let map = EditorService::batch_load_by_ids(&editor_ids, state).await?;
                        for vo in list.iter_mut() {
                            vo.editor = map.get(&vo.editor_id).cloned();
                        }
                    }
                    _ => {}
                }
            }
        }

        let vo = PaginateResultVO {
            page_size,
            current_page: page,
            total: total.number_of_items,
            list: list,
        };

        // let editor_ids = models
        //     .into_iter()
        //     .map(|model| model.editor_id)
        //     .collect::<Vec<i64>>();
        handle_ok(vo)
    }

    /// 构建列表查询器
    async fn query_builder(
        platform: &PlatformEnum,
        dto: &AppQueryDTO,
    ) -> HandleResult<Select<AppEntity>> {
        let mut query = Self::scope_active_query();
        query = query.order_by_desc(AppColumn::Id);

        if let Some(keyword) = dto.keyword.clone() {
            let condition = Condition::any()
                .add(AppColumn::Name.contains(&keyword))
                .add(AppColumn::Title.contains(&keyword));
            query = query.filter(condition);
        }

        if let Some(title) = dto.title.clone() {
            query = query.filter(AppColumn::Title.contains(&title));
        }

        if *platform == PlatformEnum::Open {
            query = query.filter(AppColumn::IsEnabled.eq(true));
        } else if let Some(enabled) = dto.is_enabled {
            query = query.filter(AppColumn::IsEnabled.eq(enabled));
        }

        if let Some(time) = dto.created_start_time.clone() {
            query = query.filter(AppColumn::CreatedAt.gte(time));
        }

        if let Some(time) = dto.created_end_time.clone() {
            query = query.filter(AppColumn::CreatedAt.lte(time));
        }

        handle_ok(query)
    }

    /// 构建查询器
    fn scope_active_query() -> Select<AppEntity> {
        AppEntity::find().filter(AppColumn::IsDeleted.eq(false))
    }

    /// 根据ID查询
    pub async fn fetch_by_id(id: i64, state: &AppState) -> HandleResult<AppModel> {
        let db = &state.db;
        let model = Self::scope_active_query()
            .filter(AppColumn::Id.eq(id))
            .one(db)
            .await?
            .ok_or_else(|| AppError::NotFound(String::from("访问记录不存在")))?;

        handle_ok(model)
    }

    /// 软删除记录
    pub async fn logic_delete(dto: &ModelLogicDeleteDTO, state: &AppState) -> HandleResult<()> {
        if dto.id < 1 {
            return handle_ok(());
        }

        let db = &state.db;
        let result = Self::fetch_by_id(dto.id, state).await;
        if let Ok(model) = result {
            let mut model: AppActiveModel = model.into();
            model.editor_type = Set(dto.editor_type.as_value());
            model.editor_id = Set(dto.editor_id);
            model.is_deleted = Set(true);
            let now = time::current_time();
            model.deleted_at = Set(Some(now));
            let _ = model.save(db).await?;
        }

        handle_ok(())
    }

    /// SelectOptionItem 列表
    pub async fn fetch_option_list(
        platform: &PlatformEnum,
        state: &AppState,
    ) -> HandleResult<Vec<SelectOptionItem>> {
        let db = &state.db;
        let mut query = Self::scope_active_query();
        if *platform == PlatformEnum::Open {
            query = query.filter(AppColumn::IsEnabled.eq(true));
        }
        let models = query.all(db).await?;
        let list: Vec<SelectOptionItem> = models.into_iter().map(|model| model.into()).collect();
        handle_ok(list)
    }

    /// 查询关联的单个记录
    pub async fn load_by_id(id: i64, state: &AppState) -> HandleResult<Option<AppLoadVO>> {
        if id < 1 {
            return handle_ok(None);
        }

        let db = &state.db;
        let model = AppEntity::find()
            .filter(AppColumn::Id.eq(id))
            .one(db)
            .await?;
        if let Some(model) = model {
            let vo: AppLoadVO = model.into();
            handle_ok(Some(vo))
        } else {
            handle_ok(None)
        }
    }

    /// 批量查询关联的记录
    pub async fn batch_load_by_ids(
        ids: &Vec<i64>,
        state: &AppState,
    ) -> HandleResult<HashMap<i64, AppLoadVO>> {
        let filted_ids: Vec<i64> = ids.into_iter().filter(|&&id| id > 0).cloned().collect();
        if filted_ids.is_empty() {
            return handle_ok(HashMap::<i64, AppLoadVO>::new());
        }

        let db = &state.db;
        let models = AppEntity::find()
            .filter(AppColumn::Id.is_in(filted_ids))
            .all(db)
            .await?;

        let map: HashMap<i64, AppLoadVO> = models
            .into_iter()
            .map(|model| (model.id, model.into()))
            .collect();

        handle_ok(map)
    }
}
