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

use crate::domain::dto::{ItemQueryDTO, ItemStoreDTO, ItemViewDTO};
use crate::domain::entity::item::{
    ActiveModel as ItemActiveModel, Column as ItemColumn, Entity as ItemEntity, Model as ItemModel,
};
use crate::domain::vo::{ItemFormOptionVO, ItemLoadVO, ItemMasterVO, ItemQueryOptionVO};
use crate::enums::ItemLoadEnum;

use super::{AppService, KindService};

pub struct ItemService {}

impl ItemService {
    /// 创建/更新
    pub async fn store(
        _platform: &PlatformEnum,
        dto: &ItemStoreDTO,
        state: &AppState,
    ) -> HandleResult<ItemModel> {
        let id: i64 = dto.id;
        let is_create = if id > 0 { false } else { true };

        let mut current_version_no = 0;
        let mut model = if is_create {
            ItemActiveModel {
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

        let mut kind_id: i64 = 0;
        if let Some(opt_kind_id) = dto.kind_id.clone() {
            kind_id = opt_kind_id;
        }
        if kind_id < 1 {
            let err = AppError::BadRequest(String::from("参数 kind_id 必须大于0"));
            return Err(err);
        }
        let kind = KindService::fetch_by_id(kind_id, state).await;
        if kind.is_err() {
            let err = AppError::BadRequest(String::from("类型不存在"));
            return Err(err);
        }
        let kind = kind.unwrap();
        if kind.is_enabled == false {
            let err = AppError::BadRequest(String::from("类型 未启用"));
            return Err(err);
        }
        model.app_id = Set(kind.app_id);
        model.kind_id = Set(kind_id);

        let mut parent_id: i64 = 0;
        if let Some(pid) = dto.parent_id.clone() {
            parent_id = pid;
            if parent_id > 0 {
                let parent = Self::fetch_by_id(parent_id, state).await?;
                if parent.is_enabled == false {
                    let err = AppError::BadRequest(String::from("父级 未启用"));
                    return Err(err);
                }
                model.parent_id = Set(parent_id);
            } else {
                parent_id = 0;
                model.parent_id = Set(0);
            }
        }

        let mut filter_extends = HashMap::<String, String>::new();
        filter_extends.insert("kind_id".to_string(), kind_id.to_string());
        filter_extends.insert("parent_id".to_string(), parent_id.to_string());

        if let Some(name) = dto.name.clone() {
            let is_exists = Self::is_column_exist(
                id,
                ItemColumn::Name,
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
                ItemColumn::Title,
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

        if let Some(introduction) = dto.introduction.clone() {
            model.introduction = Set(Some(introduction));
        }

        if let Some(icon) = dto.icon.clone() {
            model.icon = Set(icon);
        }

        if let Some(pc_detail_path) = dto.pc_detail_path.clone() {
            model.pc_detail_path = Set(pc_detail_path);
        }

        if let Some(wap_detail_path) = dto.wap_detail_path.clone() {
            model.wap_detail_path = Set(wap_detail_path);
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
        column: ItemColumn,
        value: sea_orm::Value,
        extends: &HashMap<String, String>,
        db: &DatabaseConnection,
    ) -> HandleResult<bool> {
        let mut query = Self::scope_active_query()
            .select_only()
            .column(ItemColumn::Id)
            .filter(column.eq(value));

        if let Some(id_str) = extends.get("kind_id") {
            let kind_id = id_str.parse::<i64>().unwrap_or(0);
            if kind_id > 0 {
                query = query.filter(ItemColumn::KindId.eq(kind_id));
            }
        }

        if let Some(id_str) = extends.get("parent_id") {
            let parent_id = id_str.parse::<i64>().unwrap_or(0);
            if parent_id > 0 {
                query = query.filter(ItemColumn::ParentId.eq(parent_id));
            } else {
                query = query.filter(ItemColumn::ParentId.eq(0));
            }
        }

        if id > 0 {
            query = query.filter(ItemColumn::Id.ne(id));
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
            "name" => ItemColumn::Name,
            "title" => ItemColumn::Title,
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
    ) -> HandleResult<ItemQueryOptionVO> {
        let apps = AppService::fetch_option_list(platform, state).await?;
        let kinds = KindService::fetch_option_list(platform, state).await?;
        let parents = Self::fetch_root_option_list(platform, state).await?;

        let mut vo = ItemQueryOptionVO {
            apps: Some(apps),
            kinds: Some(kinds),
            parents: Some(parents),
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
    ) -> HandleResult<ItemFormOptionVO> {
        let apps = AppService::fetch_option_list(platform, state).await?;
        let kinds = KindService::fetch_option_list(platform, state).await?;
        let parents = Self::fetch_root_option_list(platform, state).await?;

        let mut vo = ItemFormOptionVO {
            apps: Some(apps),
            kinds: Some(kinds),
            parents: Some(parents),
            enables: None,
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
        let mut model: ItemActiveModel = model.into();

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
        dto: &ItemViewDTO,
        state: &AppState,
    ) -> HandleResult<ItemMasterVO> {
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

        let mut vo: ItemMasterVO = model.into();
        if let Some(load_models) = dto.load_models.clone() {
            for enums in load_models {
                match enums {
                    ItemLoadEnum::Editor => {
                        vo.editor = EditorService::load_by_id(vo.editor_id.clone(), state).await?;
                    }
                    ItemLoadEnum::App => {
                        vo.app = AppService::load_by_id(vo.app_id.clone(), state).await?;
                    }
                    ItemLoadEnum::Kind => {
                        vo.kind = KindService::load_by_id(vo.kind_id.clone(), state).await?;
                    }
                    ItemLoadEnum::Parent => {
                        vo.parent = Self::load_by_id(vo.parent_id.clone(), state).await?;
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
        dto: &ItemQueryDTO,
        state: &AppState,
    ) -> HandleResult<PaginateResultVO<ItemMasterVO>> {
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
        let mut list: Vec<ItemMasterVO> = Vec::with_capacity(len);
        let mut editor_ids: Vec<i64> = Vec::with_capacity(len);
        let mut app_ids: Vec<i64> = Vec::with_capacity(len);
        let mut kind_ids: Vec<i64> = Vec::with_capacity(len);
        let mut parent_ids: Vec<i64> = Vec::with_capacity(len);
        for model in models.iter() {
            editor_ids.push(model.editor_id);
            app_ids.push(model.app_id);
            kind_ids.push(model.kind_id);
            parent_ids.push(model.parent_id);
            let vo: ItemMasterVO = model.into();
            list.push(vo);
        }

        if let Some(load_models) = dto.load_models.clone() {
            for enums in load_models {
                match enums {
                    ItemLoadEnum::Editor => {
                        let map = EditorService::batch_load_by_ids(&editor_ids, state).await?;
                        for vo in list.iter_mut() {
                            vo.editor = map.get(&vo.editor_id).cloned();
                        }
                    }
                    ItemLoadEnum::App => {
                        let map = AppService::batch_load_by_ids(&app_ids, state).await?;
                        for vo in list.iter_mut() {
                            vo.app = map.get(&vo.app_id).cloned();
                        }
                    }
                    ItemLoadEnum::Kind => {
                        let map = KindService::batch_load_by_ids(&kind_ids, state).await?;
                        for vo in list.iter_mut() {
                            vo.kind = map.get(&vo.kind_id).cloned();
                        }
                    }
                    ItemLoadEnum::Parent => {
                        let map = Self::batch_load_by_ids(&parent_ids, state).await?;
                        for vo in list.iter_mut() {
                            vo.parent = map.get(&vo.parent_id).cloned();
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

        handle_ok(vo)
    }

    /// 构建列表查询器
    async fn query_builder(
        platform: &PlatformEnum,
        dto: &ItemQueryDTO,
    ) -> HandleResult<Select<ItemEntity>> {
        let mut query = Self::scope_active_query();
        query = query.order_by_desc(ItemColumn::Id);

        if let Some(keyword) = dto.keyword.clone() {
            let condition = Condition::any()
                .add(ItemColumn::Name.contains(&keyword))
                .add(ItemColumn::Title.contains(&keyword));
            query = query.filter(condition);
        }

        if let Some(app_id) = dto.app_id.clone() {
            query = query.filter(ItemColumn::AppId.eq(app_id));
        }

        if let Some(kind_id) = dto.kind_id.clone() {
            query = query.filter(ItemColumn::KindId.eq(kind_id));
        }

        if let Some(parent_id) = dto.parent_id.clone() {
            let pid = if parent_id > 0 { parent_id } else { 0 };
            query = query.filter(ItemColumn::ParentId.eq(pid));
        }

        if let Some(title) = dto.title.clone() {
            query = query.filter(ItemColumn::Title.contains(&title));
        }

        if *platform == PlatformEnum::Open {
            query = query.filter(ItemColumn::IsEnabled.eq(true));
        } else if let Some(enabled) = dto.is_enabled {
            query = query.filter(ItemColumn::IsEnabled.eq(enabled));
        }

        if let Some(time) = dto.created_start_time.clone() {
            query = query.filter(ItemColumn::CreatedAt.gte(time));
        }

        if let Some(time) = dto.created_end_time.clone() {
            query = query.filter(ItemColumn::CreatedAt.lte(time));
        }

        handle_ok(query)
    }

    /// 构建查询器
    fn scope_active_query() -> Select<ItemEntity> {
        ItemEntity::find().filter(ItemColumn::IsDeleted.eq(false))
    }

    /// 根据ID查询
    pub async fn fetch_by_id(id: i64, state: &AppState) -> HandleResult<ItemModel> {
        let db = &state.db;
        let model = Self::scope_active_query()
            .filter(ItemColumn::Id.eq(id))
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
        if let Ok(model) = result {
            let mut model: ItemActiveModel = model.into();
            model.editor_type = Set(dto.editor_type.as_value());
            model.editor_id = Set(dto.editor_id);
            model.is_deleted = Set(true);
            let now = time::current_time();
            model.deleted_at = Set(Some(now));
            let _ = model.save(db).await?;
        }

        handle_ok(())
    }

    /// 根据ID查询
    pub async fn fetch_root_option_list(
        platform: &PlatformEnum,
        state: &AppState,
    ) -> HandleResult<Vec<SelectOptionItem>> {
        let db = &state.db;
        let mut query = Self::scope_active_query();
        if *platform == PlatformEnum::Open {
            query = query.filter(ItemColumn::IsEnabled.eq(true));
        }
        query = query.filter(ItemColumn::ParentId.eq(0));
        let models = query.all(db).await?;
        let list: Vec<SelectOptionItem> = models.into_iter().map(|model| model.into()).collect();
        handle_ok(list)
    }

    /// SelectOptionItem 列表
    pub async fn fetch_option_list(
        platform: &PlatformEnum,
        state: &AppState,
    ) -> HandleResult<Vec<SelectOptionItem>> {
        let db = &state.db;
        let mut query = Self::scope_active_query();
        if *platform == PlatformEnum::Open {
            query = query.filter(ItemColumn::IsEnabled.eq(true));
        }
        let models = query.all(db).await?;
        let list: Vec<SelectOptionItem> = models.into_iter().map(|model| model.into()).collect();
        handle_ok(list)
    }

    /// 查询关联的单个记录
    pub async fn load_by_id(id: i64, state: &AppState) -> HandleResult<Option<ItemLoadVO>> {
        if id < 1 {
            return handle_ok(None);
        }

        let db = &state.db;
        let model = ItemEntity::find()
            .filter(ItemColumn::Id.eq(id))
            .one(db)
            .await?;
        if let Some(model) = model {
            let vo: ItemLoadVO = model.into();
            handle_ok(Some(vo))
        } else {
            handle_ok(None)
        }
    }

    /// 批量查询关联的记录
    pub async fn batch_load_by_ids(
        ids: &Vec<i64>,
        state: &AppState,
    ) -> HandleResult<HashMap<i64, ItemLoadVO>> {
        let filted_ids: Vec<i64> = ids.into_iter().filter(|&&id| id > 0).cloned().collect();
        if filted_ids.is_empty() {
            return handle_ok(HashMap::<i64, ItemLoadVO>::new());
        }

        let db = &state.db;
        let models = ItemEntity::find()
            .filter(ItemColumn::Id.is_in(filted_ids))
            .all(db)
            .await?;

        let map: HashMap<i64, ItemLoadVO> = models
            .into_iter()
            .map(|model| (model.id, model.into()))
            .collect();

        handle_ok(map)
    }
}
