use sea_orm::*;
use std::collections::HashMap;

use cms_core::config::AppState;
use cms_core::domain::{
    HandleResult,
    dto::{
        EditorCurrentDTO, FieldBoolUpdateDTO, FieldValueUniqueDTO, ModelLogicDeleteDTO,
        ModelViewDTO,
    },
    handle_ok,
    model::SelectOptionModel,
    vo::PaginateResultVO,
};
use cms_core::enums::{
    EditorTypeEnum, EnableEnum, ErrorEnum as CoreErrorEnum, PlatformEnum, ViewModeEnum,
};
use cms_core::error::AppError;
use cms_core::service::{EditorService, RedisService};
use cms_core::utils::time_utils;

use crate::domain::dto::{AppQueryDTO, AppStoreDTO};
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
        let id = dto.id;
        let is_create = id <= 0;

        let (current_version_no, mut model) = if is_create {
            (
                0,
                AppActiveModel {
                    ..Default::default()
                },
            )
        } else {
            let model = Self::fetch_by_id(id, state).await?;
            let current_version_no = model.version_no.unwrap_or(0);
            (current_version_no, model.into())
        };

        // 检查版本号
        if let Some(version_no) = dto.version_no {
            if !is_create && version_no != current_version_no {
                return Err(Into::<AppError>::into(CoreErrorEnum::VersionNoInvalid));
            }
        }
        model.version_no = Set(Some(current_version_no + 1));

        let db = &state.db;
        let filter_extends = HashMap::new();

        if let Some(name) = &dto.name {
            if Self::is_column_exist(
                id,
                AppColumn::Name,
                sea_orm::Value::from(name),
                &filter_extends,
                db,
            )
            .await?
            {
                return Err(Into::<AppError>::into(CoreErrorEnum::NameExists));
            }
            model.name = Set(name.clone());
        }

        if let Some(title) = &dto.title {
            if Self::is_column_exist(
                id,
                AppColumn::Title,
                sea_orm::Value::from(title),
                &filter_extends,
                db,
            )
            .await?
            {
                return Err(Into::<AppError>::into(CoreErrorEnum::TitleExists));
            }
            model.title = Set(title.clone());
        }

        if let Some(description) = &dto.description {
            model.description = Set(description.clone());
        }

        if let Some(icon) = &dto.icon {
            model.icon = Set(icon.clone());
        }

        if let Some(sort) = dto.sort {
            model.sort = Set(sort);
        }

        if let Some(is_enabled) = dto.is_enabled {
            model.is_enabled = Set(is_enabled);
        }

        let time = time_utils::current_time();
        model.updated_at = Set(Some(time));

        if is_create {
            model.created_at = Set(Some(time));
        }

        let editor = dto.editor.clone();
        model.editor_type = Set(editor.editor_type.string_value());
        // model.editor_id = Set(editor.editor_id);

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
        let _id = dto.skip_id;
        let db = &state.db;

        let column = match dto.field_name.to_lowercase().as_str() {
            "name" => AppColumn::Name,
            "title" => AppColumn::Title,
            _ => return Err(Into::<AppError>::into(CoreErrorEnum::FieldInvalid)),
        };

        let value = sea_orm::Value::from(dto.field_value.clone());
        let filter_extends = dto.extends.clone().unwrap_or_default();
        let exist = Self::is_column_exist(0, column, value, &filter_extends, db).await?;
        handle_ok(!exist)
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
            return Err(Into::<AppError>::into(CoreErrorEnum::ParamIdInvalid));
        }
        let db = &state.db;

        let model = Self::fetch_by_id(id, state).await?;
        let mut model: AppActiveModel = model.into();

        let bool_value = dto.field_value;
        match dto.field_name.to_lowercase().as_str() {
            "is_enabled" | "enabled" => model.is_enabled = Set(bool_value),
            _ => {
                return Err(Into::<AppError>::into(CoreErrorEnum::UpdateFieldInvalid));
            }
        };

        let now = time_utils::current_time();
        model.updated_at = Set(Some(now));

        let editor = dto.editor.clone();
        model.editor_type = Set(editor.editor_type.string_value());
        // model.editor_id = Set(editor.editor_id);

        let _ = model.save(db).await?;

        handle_ok(true)
    }

    /// 查看
    pub async fn view(
        platform: &PlatformEnum,
        dto: &ModelViewDTO<AppLoadEnum>,
        state: &AppState,
    ) -> HandleResult<AppMasterVO> {
        let id = dto.id;
        if id < 1 {
            return Err(Into::<AppError>::into(CoreErrorEnum::ParamIdInvalid));
        }

        let model = Self::fetch_by_id(id, state).await?;

        let view_enum = ViewModeEnum::platform_to_detail_mode(platform);
        if view_enum == ViewModeEnum::OpenDetail && !model.is_enabled {
            return Err(Into::<AppError>::into(CoreErrorEnum::RecordNotFound));
        }

        let mut vo: AppMasterVO = AppMasterVO::mode_into(&view_enum, &model);
        if let Some(ref load_models) = dto.load_models {
            for enums in load_models {
                match enums {
                    // AppLoadEnum::Editor => {
                    //     vo.editor = EditorService::load_by_id(vo.editor_id.clone(), state).await?;
                    // }
                    _ => {}
                }
            }
        }

        let editor = dto.editor.clone();
        if view_enum == ViewModeEnum::ManagerDetail {
            vo.can_update = Some(true);
            vo.can_delete = Some(Self::can_delete(&editor, &model));
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
        let page = dto.page;
        let page_size = dto.page_size;

        let editor = dto.editor.clone();
        let view_enum = ViewModeEnum::platform_to_list_mode(platform);

        let mut cols = vec![
            AppColumn::Id,
            AppColumn::EditorId,
            AppColumn::EditorType,
            AppColumn::Name,
            AppColumn::Title,
            AppColumn::Description,
            AppColumn::Icon,
            AppColumn::Sort,
            AppColumn::IsEnabled,
        ];
        if view_enum == ViewModeEnum::ManagerList {
            cols.push(AppColumn::CreatedAt);
            cols.push(AppColumn::UpdatedAt);
        }

        let mut query = Self::query_builder(platform, dto).await?;
        query = query.select_only().columns(cols);
        let paginator = query.paginate(db, page_size);
        let total = paginator.num_items_and_pages().await?;
        let models = paginator.fetch_page(page - 1).await?;
        let len = models.len();
        let mut list: Vec<AppMasterVO> = Vec::with_capacity(len);
        let mut editor_ids: Vec<i64> = Vec::with_capacity(len);
        for model in models.iter() {
            editor_ids.push(model.editor_id);
            let mut vo: AppMasterVO = AppMasterVO::mode_into(&view_enum, &model);
            if view_enum == ViewModeEnum::ManagerList {
                vo.can_update = Some(true);
                vo.can_delete = Some(Self::can_delete(&editor, &model));
            }
            list.push(vo);
        }

        if let Some(ref load_models) = dto.load_models {
            for enums in load_models {
                match enums {
                    // AppLoadEnum::Editor => {
                    //     let map = EditorService::batch_load_by_ids(&editor_ids, state).await?;
                    //     for vo in list.iter_mut() {
                    //         vo.editor = map.get(&vo.editor_id).cloned();
                    //     }
                    // }
                    _ => {}
                }
            }
        }

        let vo = PaginateResultVO {
            page_size,
            current_page: page,
            total: total.number_of_items,
            list,
        };

        handle_ok(vo)
    }

    /// 构建列表查询器
    async fn query_builder(
        platform: &PlatformEnum,
        dto: &AppQueryDTO,
    ) -> HandleResult<Select<AppEntity>> {
        let mut query = Self::scope_active_query();
        query = query.order_by_desc(AppColumn::Id);

        if let Some(ref keyword) = dto.keyword {
            let condition = Condition::any()
                .add(AppColumn::Name.contains(keyword))
                .add(AppColumn::Title.contains(keyword));
            query = query.filter(condition);
        }

        if let Some(ref title) = dto.title {
            query = query.filter(AppColumn::Title.contains(title));
        }

        if *platform == PlatformEnum::Open {
            query = query.filter(AppColumn::IsEnabled.eq(true));
        } else if let Some(enabled) = dto.is_enabled {
            query = query.filter(AppColumn::IsEnabled.eq(enabled));
        }

        if let Some(ref time) = dto.created_start_time {
            query = query.filter(AppColumn::CreatedAt.gte(*time));
        }

        if let Some(ref time) = dto.created_end_time {
            query = query.filter(AppColumn::CreatedAt.lte(*time));
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
            .ok_or_else(|| Into::<AppError>::into(CoreErrorEnum::ParamIdInvalid))?;

        handle_ok(model)
    }

    /// 软删除记录
    pub async fn logic_delete(dto: &ModelLogicDeleteDTO, state: &AppState) -> HandleResult<()> {
        if dto.id < 1 {
            return handle_ok(());
        }

        let db = &state.db;
        let result = Self::fetch_by_id(dto.id, state).await;
        if result.is_err() {
            return handle_ok(());
        }
        let model = result.unwrap();
        let editor = dto.editor.clone();
        if !Self::can_delete(&editor, &model) {
            return Err(Into::<AppError>::into(CoreErrorEnum::NoPermissionDelete));
        }
        let mut model: AppActiveModel = model.into();
        model.editor_type = Set(editor.editor_type.string_value());
        // model.editor_id = Set(editor.editor_id);
        model.is_deleted = Set(Some(true));
        let now = time_utils::current_time();
        model.deleted_at = Set(Some(now));
        let _ = model.save(db).await?;

        handle_ok(())
    }

    /// SelectOptionModel 列表
    pub async fn fetch_option_list(
        platform: &PlatformEnum,
        state: &AppState,
    ) -> HandleResult<Vec<SelectOptionModel>> {
        let cache_key = "mate:app:option_list";
        let only_enabled = *platform == PlatformEnum::Open;

        let stored_list =
            RedisService::get_json_list::<SelectOptionModel>(&state.redis, cache_key).await;
        if !stored_list.is_empty() {
            if only_enabled {
                let filted_list = stored_list
                    .into_iter()
                    .filter(|opt| !opt.disabled.unwrap_or(false))
                    .collect();

                return handle_ok(filted_list);
            } else {
                return handle_ok(stored_list);
            }
        }

        let db = &state.db;
        let mut query = Self::scope_active_query();
        if *platform == PlatformEnum::Open {
            query = query.filter(AppColumn::IsEnabled.eq(true));
        }
        let models = query.all(db).await?;
        let list: Vec<SelectOptionModel> = models.into_iter().map(|model| model.into()).collect();
        RedisService::set_json_list::<SelectOptionModel>(&state.redis, cache_key, &list).await;

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
            return handle_ok(HashMap::new());
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

    /// 是否可以删除记录
    pub fn can_delete(editor: &EditorCurrentDTO, model: &AppModel) -> bool {
        if let Some(num) = model.kind_count {
            if num > 0 {
                return false;
            }
        } else {
            return false;
        }

        editor.editor_type == EditorTypeEnum::Admin
    }
}
