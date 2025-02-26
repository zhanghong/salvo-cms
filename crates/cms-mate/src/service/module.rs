use sea_orm::prelude::Expr;
use sea_orm::*;

use cms_core::config::AppState;
use cms_core::domain::{
    HandleResult,
    dto::{FieldBoolUpdateDTO, FieldValueUniqueDTO},
    handle_ok,
    vo::PaginateResultVO,
};
use cms_core::enums::{EnableEnum, PlatformEnum};
use cms_core::error::AppError;
use cms_core::service::EditorService;
use cms_core::utils::time;

use crate::domain::dto::{ModuleQueryDTO, ModuleStoreDTO, ModuleViewDTO};
use crate::domain::entity::module::{
    ActiveModel as ModuleActiveModel, Column as ModuleColumn, Entity as ModuleEntity,
    Model as ModuleModel,
};
use crate::domain::vo::{ModuleFormOptionVO, ModuleVO};
use crate::enums::ModuleLoadEnum;

pub struct ModuleService {}

impl ModuleService {
    /// 创建/更新
    pub async fn store(dto: &ModuleStoreDTO, state: &AppState) -> HandleResult<ModuleModel> {
        let mut id: i64 = 0;
        let mut is_create = true;
        if dto.id.is_some() {
            id = dto.id.unwrap();
            is_create = false;
        }
        let mut model = if is_create {
            ModuleActiveModel {
                ..Default::default()
            }
        } else {
            let model = Self::fetch_by_id(id, state).await?;
            model.into()
        };
        let db = &state.db;

        if let Some(name) = dto.name.clone() {
            let is_exists = Self::is_column_exist(
                id,
                ModuleColumn::Name,
                sea_orm::Value::from(name.to_owned()),
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
                ModuleColumn::Title,
                sea_orm::Value::from(title.to_owned()),
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
        column: ModuleColumn,
        value: sea_orm::Value,
        db: &DatabaseConnection,
    ) -> HandleResult<bool> {
        let count = Self::scope_active_query()
            .select_only()
            .column(ModuleColumn::Id)
            .filter(column.eq(value))
            .filter(ModuleColumn::Id.ne(id))
            .count(db)
            .await?;

        handle_ok(count > 0)
    }

    /// 表单选项
    pub fn form_options() -> HandleResult<ModuleFormOptionVO> {
        let enables = EnableEnum::to_option_list();

        let vo = ModuleFormOptionVO { enables };

        handle_ok(vo)
    }

    /// 检查字段值是否唯一
    pub async fn field_unique(dto: &FieldValueUniqueDTO, state: &AppState) -> HandleResult<bool> {
        let id = dto.skip_id;
        let db = &state.db;

        let field_name = dto.field_name.to_owned();
        let column = match field_name.to_lowercase().as_str() {
            "name" => ModuleColumn::Name,
            "title" => ModuleColumn::Title,
            _ => {
                let err = AppError::BadRequest(String::from("无效的字段"));
                return Err(err);
            }
        };

        let field_value = dto.field_value.to_owned();
        let value = sea_orm::Value::from(field_value);

        let exist = Self::is_column_exist(id, column, value, db).await?;
        handle_ok(exist != true)
    }

    /// 修改布尔值字段
    pub async fn update_bool_field(
        dto: &FieldBoolUpdateDTO,
        state: &AppState,
    ) -> HandleResult<bool> {
        let id = dto.id;
        if id < 1 {
            let err = AppError::BadRequest(String::from("无效的用户ID"));
            return Err(err);
        }
        let db = &state.db;

        let field_name = dto.field_name.to_owned();
        let column = match field_name.to_lowercase().as_str() {
            "is_enabled" => ModuleColumn::IsEnabled,
            _ => {
                let err = AppError::BadRequest(String::from("无效的字段"));
                return Err(err);
            }
        };

        let _update_rows_count = ModuleEntity::update_many()
            .col_expr(column, Expr::value(dto.field_value))
            .filter(ModuleColumn::Id.eq(id))
            .exec(db)
            .await?;

        handle_ok(true)
    }

    /// 查看
    pub async fn view(
        platform: &PlatformEnum,
        dto: &ModuleViewDTO,
        state: &AppState,
    ) -> HandleResult<ModuleVO> {
        let id = dto.id;
        if id < 1 {
            let err = AppError::BadRequest(String::from("访问记录不存在"));
            return Err(err);
        }

        let model = Self::fetch_by_id(id, state).await?;
        if *platform == PlatformEnum::Open {
            if !model.is_enabled {
                let err = AppError::BadRequest(String::from("访问记录不存在"));
                return Err(err);
            }
        }

        let mut vo: ModuleVO = model.into();
        if let Some(load_models) = dto.load_models.clone() {
            for enums in load_models {
                match enums {
                    ModuleLoadEnum::Editor => {
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
        dto: &ModuleQueryDTO,
        state: &AppState,
    ) -> HandleResult<PaginateResultVO<ModuleVO>> {
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
        let mut list: Vec<ModuleVO> = Vec::with_capacity(len);
        let mut editor_ids: Vec<i64> = Vec::with_capacity(len);
        for model in models.iter() {
            editor_ids.push(model.editor_id);
            let vo: ModuleVO = model.into();
            list.push(vo);
        }

        if let Some(load_models) = dto.load_models.clone() {
            for enums in load_models {
                match enums {
                    ModuleLoadEnum::Editor => {
                        let map = EditorService::batch_load_by_ids(&editor_ids, state).await?;
                        for vo in list.iter_mut() {
                            let editor = map.get(&vo.editor_id).cloned();
                            vo.editor = editor;
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
        dto: &ModuleQueryDTO,
    ) -> HandleResult<Select<ModuleEntity>> {
        let mut query = Self::scope_active_query();
        query = query.order_by_desc(ModuleColumn::Id);

        if let Some(keyword) = dto.keyword.clone() {
            let condition = Condition::any()
                .add(ModuleColumn::Name.contains(&keyword))
                .add(ModuleColumn::Title.contains(&keyword));
            query = query.filter(condition);
        }

        if let Some(title) = dto.title.clone() {
            query = query.filter(ModuleColumn::Title.contains(&title));
        }

        if *platform == PlatformEnum::Open {
            query = query.filter(ModuleColumn::IsEnabled.eq(true));
        } else if let Some(enabled) = dto.is_enabled {
            query = query.filter(ModuleColumn::IsEnabled.eq(enabled));
        }

        if let Some(time) = dto.created_start_time.clone() {
            query = query.filter(ModuleColumn::CreatedAt.gte(time));
        }

        if let Some(time) = dto.created_end_time.clone() {
            query = query.filter(ModuleColumn::CreatedAt.lte(time));
        }

        handle_ok(query)
    }

    /// 构建查询器
    fn scope_active_query() -> Select<ModuleEntity> {
        ModuleEntity::find().filter(ModuleColumn::IsDeleted.eq(false))
    }

    /// 根据ID查询
    async fn fetch_by_id(id: i64, state: &AppState) -> HandleResult<ModuleModel> {
        let db = &state.db;
        let model = Self::scope_active_query()
            .filter(ModuleColumn::Id.eq(id))
            .one(db)
            .await?
            .ok_or_else(|| AppError::BadRequest(String::from("无效的ID")))?;

        handle_ok(model)
    }

    /// 软删除记录
    pub async fn destroy(id: i64, state: &AppState) -> HandleResult<()> {
        if id < 1 {
            return handle_ok(());
        }

        let db = &state.db;
        let result = Self::fetch_by_id(id, state).await;
        if let Ok(model) = result {
            let mut model: ModuleActiveModel = model.into();
            model.is_deleted = Set(true);
            let now = time::current_time();
            model.deleted_at = Set(Some(now));
            let _ = model.save(db).await?;
        }

        handle_ok(())
    }
}
