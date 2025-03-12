use sea_orm::prelude::Expr;
use sea_orm::*;
use std::collections::HashMap;

use cms_core::config::AppState;
use cms_core::domain::{
    HandleResult, SelectOptionItem, SelectValueEnum,
    dto::{
        EditorCurrent, FieldBoolUpdateDTO, FieldValueUniqueDTO, ModelLogicDeleteDTO,
        ModelRelationCountDTO, ModelViewDTO,
    },
    handle_ok,
    vo::PaginateResultVO,
};
use cms_core::enums::{EditorTypeEnum, EnableEnum, PlatformEnum, ViewModeEnum};
use cms_core::error::AppError;
use cms_core::service::EditorService;
use cms_core::utils::time;

use crate::domain::dto::{KindQueryDTO, KindStoreDTO};
use crate::domain::entity::app::{Column as AppColumn, Entity as AppEntity};
use crate::domain::entity::kind::{
    ActiveModel as KindActiveModel, Column as KindColumn, Entity as KindEntity, Model as KindModel,
};
use crate::domain::vo::{KindFormOptionVO, KindLoadVO, KindMasterVO, KindQueryOptionVO};
use crate::enums::KindLoadEnum;

use super::AppService;

pub struct KindService {}

impl KindService {
    /// 创建/更新
    pub async fn store(
        _platform: &PlatformEnum,
        dto: &KindStoreDTO,
        state: &AppState,
    ) -> HandleResult<KindModel> {
        let id: i64 = dto.id;
        let is_create = if id > 0 { false } else { true };

        let mut old_app_id = 0;
        let mut current_version_no = 0;
        let mut model = if is_create {
            KindActiveModel {
                ..Default::default()
            }
        } else {
            let model = Self::fetch_by_id(id, state).await?;
            old_app_id = model.app_id;
            if let Some(no) = model.version_no.clone() {
                current_version_no = no;
            }
            model.into()
        };

        // 检查版本号
        if let Some(version_no) = dto.version_no.clone() {
            if !is_create && version_no.ne(&current_version_no) {
                let err = AppError::BadRequest(String::from("版本号错误"));
                return Err(err);
            }
        }
        model.version_no = Set(Some(current_version_no + 1));

        let db = &state.db;

        let mut new_app_id: i64 = 0;
        if let Some(opt_app_id) = dto.app_id.clone() {
            new_app_id = opt_app_id;
        }
        if new_app_id < 1 {
            let err = AppError::BadRequest(String::from("参数 app_id 必须大于0"));
            return Err(err);
        }
        let app = AppService::fetch_by_id(new_app_id, state).await;
        if app.is_err() {
            let err = AppError::BadRequest(String::from("App 不存在"));
            return Err(err);
        }
        let app = app.unwrap();
        if app.is_enabled == false {
            let err = AppError::BadRequest(String::from("App 未启用"));
            return Err(err);
        }
        if old_app_id.ne(&new_app_id) {
            model.app_id = Set(new_app_id);
        }

        let mut filter_extends = HashMap::<String, String>::new();
        if let Some(name) = dto.name.clone() {
            filter_extends.insert("app_id".to_string(), "0".to_string());
            let is_exists = Self::is_column_exist(
                id,
                KindColumn::Name,
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

        filter_extends.insert("app_id".to_string(), new_app_id.to_string());
        if let Some(title) = dto.title.clone() {
            let is_exists = Self::is_column_exist(
                id,
                KindColumn::Title,
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

        if let Some(max_level) = dto.max_level.clone() {
            model.max_level = Set(max_level);
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
        model.updated_at = Set(Some(time));

        if is_create {
            model.created_at = Set(Some(time));
        }

        let editor = dto.editor.clone();
        model.editor_type = Set(editor.editor_type.as_value());
        model.editor_id = Set(editor.editor_id);

        let app_ids = vec![old_app_id, new_app_id];

        // 提交事务
        let txn = db.begin().await?;
        let model = model.save(&txn).await?;
        let model = model.try_into_model()?;
        let _ = Self::batch_upload_count_in_apps(app_ids, state).await?;
        txn.commit().await?;

        handle_ok(model)
    }

    /// 检查字段值是否唯一（查询）
    async fn is_column_exist(
        id: i64,
        column: KindColumn,
        value: sea_orm::Value,
        extends: &HashMap<String, String>,
        db: &DatabaseConnection,
    ) -> HandleResult<bool> {
        let mut query = Self::scope_active_query()
            .select_only()
            .column(KindColumn::Id)
            .filter(column.eq(value));

        if let Some(id_str) = extends.get("app_id") {
            let app_id = id_str.parse::<i64>().unwrap_or(0);
            if app_id > 0 {
                query = query.filter(KindColumn::AppId.eq(app_id));
            }
        }

        if id > 0 {
            query = query.filter(KindColumn::Id.ne(id));
        }
        let count = query.count(db).await?;

        handle_ok(count > 0)
    }

    /// 检查字段值是否唯一
    pub async fn field_unique(dto: &FieldValueUniqueDTO, state: &AppState) -> HandleResult<bool> {
        let id = dto.skip_id;
        let db = &state.db;

        let field_name = dto.field_name.to_owned();
        let column = match field_name.to_lowercase().as_str() {
            "name" => KindColumn::Name,
            "title" => KindColumn::Title,
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
        state: &AppState,
    ) -> HandleResult<KindQueryOptionVO> {
        let apps = AppService::fetch_option_list(platform, state).await?;

        let mut vo = KindQueryOptionVO {
            apps: Some(apps),
            enables: None,
        };
        if *platform == PlatformEnum::Manager {
            let enables = EnableEnum::to_option_list();
            vo.enables = Some(enables);
        }

        handle_ok(vo)
    }

    /// 表单选项
    pub async fn form_options(
        platform: &PlatformEnum,
        state: &AppState,
    ) -> HandleResult<KindFormOptionVO> {
        let apps = AppService::fetch_option_list(platform, state).await?;

        let mut vo = KindFormOptionVO {
            enables: None,
            apps: apps,
        };

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
        let mut model: KindActiveModel = model.into();

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
        model.updated_at = Set(Some(now));

        let editor = dto.editor.clone();
        model.editor_type = Set(editor.editor_type.as_value());
        model.editor_id = Set(editor.editor_id);

        let _ = model.save(db).await?;

        handle_ok(true)
    }

    /// 查看
    pub async fn view(
        platform: &PlatformEnum,
        dto: &ModelViewDTO<KindLoadEnum>,
        state: &AppState,
    ) -> HandleResult<KindMasterVO> {
        let id = dto.id;
        if id < 1 {
            let err = AppError::BadRequest(String::from("参数ID错误"));
            return Err(err);
        }

        let model = Self::fetch_by_id(id, state).await?;

        let view_enum = ViewModeEnum::platform_to_detail_mode(platform);
        if view_enum == ViewModeEnum::OpenDetail {
            if !model.is_enabled {
                let err = AppError::NotFound(String::from("访问记录不存在"));
                return Err(err);
            }
        }

        let mut vo: KindMasterVO = KindMasterVO::mode_into(&view_enum, &model);
        if let Some(load_models) = dto.load_models.clone() {
            for enums in load_models {
                match enums {
                    KindLoadEnum::Editor => {
                        vo.editor = EditorService::load_by_id(vo.editor_id.clone(), state).await?;
                    }
                    KindLoadEnum::App => {
                        vo.app = AppService::load_by_id(vo.app_id.clone(), state).await?;
                    }
                    _ => {}
                }
            }
        }

        let editor = dto.editor.clone();
        if *platform == PlatformEnum::Manager {
            vo.can_update = Some(true);
            vo.can_delete = Some(Self::can_delete(&editor, &model));
        }

        handle_ok(vo)
    }

    /// 分页查询
    pub async fn paginage(
        platform: &PlatformEnum,
        dto: &KindQueryDTO,
        state: &AppState,
    ) -> HandleResult<PaginateResultVO<KindMasterVO>> {
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

        let editor = dto.editor.clone();
        let view_enum = ViewModeEnum::platform_to_list_mode(platform);

        let mut cols = vec![
            KindColumn::Id,
            KindColumn::EditorId,
            KindColumn::EditorType,
            KindColumn::AppId,
            KindColumn::Name,
            KindColumn::Title,
            KindColumn::MaxLevel,
            KindColumn::Description,
            KindColumn::Icon,
            KindColumn::IsMultiple,
            KindColumn::Sort,
            KindColumn::IsEnabled,
        ];
        if view_enum == ViewModeEnum::ManagerList {
            cols.push(KindColumn::CreatedAt);
            cols.push(KindColumn::UpdatedAt);
        }

        let mut query = Self::query_builder(platform, dto).await?;
        query = query.select_only().columns(cols);
        let paginator = query.paginate(db, page_size);
        let total = paginator.num_items_and_pages().await?;
        let models = paginator.fetch_page(page - 1).await?;
        let len = models.len();
        let mut list: Vec<KindMasterVO> = Vec::with_capacity(len);
        let mut editor_ids: Vec<i64> = Vec::with_capacity(len);
        let mut app_ids: Vec<i64> = Vec::with_capacity(len);
        for model in models.iter() {
            editor_ids.push(model.editor_id);
            app_ids.push(model.app_id);
            let mut vo: KindMasterVO = KindMasterVO::mode_into(&view_enum, &model);
            if *platform == PlatformEnum::Manager {
                vo.can_update = Some(true);
                vo.can_delete = Some(Self::can_delete(&editor, &model));
            }
            list.push(vo);
        }

        if let Some(load_models) = dto.load_models.clone() {
            for enums in load_models {
                match enums {
                    KindLoadEnum::Editor => {
                        let map = EditorService::batch_load_by_ids(&editor_ids, state).await?;
                        for vo in list.iter_mut() {
                            vo.editor = map.get(&vo.editor_id).cloned();
                        }
                    }
                    KindLoadEnum::App => {
                        let map = AppService::batch_load_by_ids(&app_ids, state).await?;
                        for vo in list.iter_mut() {
                            vo.app = map.get(&vo.app_id).cloned();
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
        dto: &KindQueryDTO,
    ) -> HandleResult<Select<KindEntity>> {
        let mut query = Self::scope_active_query();
        query = query.order_by_desc(KindColumn::Id);

        if let Some(keyword) = dto.keyword.clone() {
            let condition = Condition::any()
                .add(KindColumn::Name.contains(&keyword))
                .add(KindColumn::Title.contains(&keyword));
            query = query.filter(condition);
        }

        if let Some(title) = dto.title.clone() {
            query = query.filter(KindColumn::Title.contains(&title));
        }

        if *platform == PlatformEnum::Open {
            query = query.filter(KindColumn::IsEnabled.eq(true));
        } else if let Some(enabled) = dto.is_enabled {
            query = query.filter(KindColumn::IsEnabled.eq(enabled));
        }

        if let Some(time) = dto.created_start_time.clone() {
            query = query.filter(KindColumn::CreatedAt.gte(time));
        }

        if let Some(time) = dto.created_end_time.clone() {
            query = query.filter(KindColumn::CreatedAt.lte(time));
        }

        handle_ok(query)
    }

    /// 构建查询器
    fn scope_active_query() -> Select<KindEntity> {
        KindEntity::find().filter(KindColumn::IsDeleted.eq(false))
    }

    /// 根据ID查询
    pub async fn fetch_by_id(id: i64, state: &AppState) -> HandleResult<KindModel> {
        let db = &state.db;
        let model = Self::scope_active_query()
            .filter(KindColumn::Id.eq(id))
            .one(db)
            .await?
            .ok_or_else(|| AppError::BadRequest(String::from("访问记录不存在")))?;

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
        if Self::can_delete(&editor, &model) == false {
            let err = AppError::BadRequest(String::from("无权限删除"));
            return Err(err);
        }
        let mut model: KindActiveModel = model.into();
        model.editor_type = Set(editor.editor_type.as_value());
        model.editor_id = Set(editor.editor_id);
        model.is_deleted = Set(Some(true));
        let now = time::current_time();
        model.deleted_at = Set(Some(now));
        let _ = model.save(db).await?;

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
            query = query.filter(KindColumn::IsEnabled.eq(true));
        }
        let models = query.all(db).await?;
        let list: Vec<SelectOptionItem> = models.into_iter().map(|model| model.into()).collect();
        handle_ok(list)
    }

    /// SelectOptionItem 列表
    pub async fn fetch_option_with_app(
        platform: &PlatformEnum,
        state: &AppState,
    ) -> HandleResult<Vec<SelectOptionItem>> {
        let mut apps = AppService::fetch_option_list(platform, state).await?;
        if apps.len() < 1 {
            return handle_ok(vec![]);
        }
        let db = &state.db;
        let mut query = Self::scope_active_query();
        if *platform == PlatformEnum::Open {
            query = query.filter(KindColumn::IsEnabled.eq(true));
        }
        let models = query.all(db).await?;
        let list = apps
            .iter_mut()
            .map(|app| {
                let mut item = app.clone();
                let app_id = match item.value {
                    SelectValueEnum::Number(id) => id,
                    _ => 0,
                };
                let mut children = vec![];
                for model in models.iter() {
                    if model.app_id == app_id {
                        let child: SelectOptionItem = model.into();
                        children.push(child);
                    }
                }
                if children.len() > 0 {
                    item.children = Some(children);
                }
                item
            })
            .collect();
        handle_ok(list)
    }

    /// 查询关联的单个记录
    pub async fn load_by_id(id: i64, state: &AppState) -> HandleResult<Option<KindLoadVO>> {
        if id < 1 {
            return handle_ok(None);
        }

        let db = &state.db;
        let model = KindEntity::find()
            .filter(KindColumn::Id.eq(id))
            .one(db)
            .await?;
        if let Some(model) = model {
            let vo: KindLoadVO = model.into();
            handle_ok(Some(vo))
        } else {
            handle_ok(None)
        }
    }

    /// 批量查询关联的记录
    pub async fn batch_load_by_ids(
        ids: &Vec<i64>,
        state: &AppState,
    ) -> HandleResult<HashMap<i64, KindLoadVO>> {
        let filted_ids: Vec<i64> = ids.into_iter().filter(|&&id| id > 0).cloned().collect();
        if filted_ids.is_empty() {
            return handle_ok(HashMap::<i64, KindLoadVO>::new());
        }

        let db = &state.db;
        let models = KindEntity::find()
            .filter(KindColumn::Id.is_in(filted_ids))
            .all(db)
            .await?;

        let map: HashMap<i64, KindLoadVO> = models
            .into_iter()
            .map(|model| (model.id, model.into()))
            .collect();

        handle_ok(map)
    }

    /// 批量更新应用的记录数量
    async fn batch_upload_count_in_apps(app_ids: Vec<i64>, state: &AppState) -> HandleResult<()> {
        let app_ids: Vec<i64> = app_ids.iter().filter(|&&id| id > 0).cloned().collect();
        if app_ids.is_empty() {
            return handle_ok(());
        }
        let db = &state.db;
        let models: Vec<ModelRelationCountDTO> = Self::scope_active_query()
            .select_only()
            .column_as(KindColumn::AppId, "relation_id")
            .column_as(KindColumn::Id.count(), "item_count")
            .filter(KindColumn::AppId.is_in(app_ids.clone()))
            .group_by(KindColumn::AppId)
            .into_model::<ModelRelationCountDTO>()
            .all(db)
            .await?;
        let map: HashMap<i64, i16> = models
            .into_iter()
            .map(|model| (model.relation_id, model.item_count))
            .collect();

        for id in app_ids.iter() {
            let count = map.get(id).unwrap_or(&0);

            let _ = AppEntity::update_many()
                .col_expr(AppColumn::KindCount, Expr::value(*count))
                .filter(AppColumn::Id.eq(*id))
                .exec(db)
                .await?;
        }

        handle_ok(())
    }

    /// 是否可以删除记录
    pub fn can_delete(editor: &EditorCurrent, model: &KindModel) -> bool {
        let has_item = match model.item_count {
            Some(count) => count > 0,
            None => false,
        };
        if has_item {
            return false;
        }

        return editor.editor_type == EditorTypeEnum::Admin;
    }
}
