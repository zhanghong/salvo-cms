use sea_orm::prelude::Expr;
use sea_orm::*;

use cms_core::config::AppState;
use cms_core::domain::{
    HandleResult,
    dto::{FieldBoolUpdateDTO, FieldValueUniqueDTO},
    handle_ok,
    vo::PaginateResultVO,
};
use cms_core::enums::{EditorTypeEnum, PlatformEnum};
use cms_core::error::AppError;
use cms_core::service::EditorService;
use cms_core::utils::{encrypt_utils::encrypt_password, random_utils, time_utils};

use crate::domain::dto::{
    DetailStoreDTO, UserQueryDTO, UserStoreDTO, UserUpdatePasswordDTO, UserViewDTO,
};
use crate::domain::entity::detail::{
    ActiveModel as DetailActiveModel, Column as DetailColumn, Entity as DetailEntity,
};
use crate::domain::entity::user::{
    ActiveModel as UserActiveModel, Column as UserColumn, Entity as UserEntity, Model as UserModel,
};
use crate::domain::vo::{UserFormOptionVO, UserItemVO};
use crate::enums::{GenderEnum, UserLoadEnum};

pub struct UserService {}

const RAND_SALT_LENGTH: usize = 5;
const RAND_NO_LENGTH: usize = 10;

impl UserService {
    pub async fn store(
        platform: &PlatformEnum,
        dto: &UserStoreDTO,
        state: &AppState,
    ) -> HandleResult<UserModel> {
        let mut id: i64 = 0;
        let mut is_create = true;
        if dto.id.is_some() {
            id = dto.id.unwrap();
            is_create = false;
        }
        let mut model = if is_create {
            UserActiveModel {
                ..Default::default()
            }
        } else {
            let model = Self::fetch_by_id(id, state).await?;
            model.into()
        };
        let db = &state.db;

        let name = dto.name.clone();
        if name.is_some() {
            let name = name.unwrap();
            let is_exists = Self::is_column_exist(
                id,
                UserColumn::Name,
                sea_orm::Value::from(name.to_owned()),
                db,
            )
            .await?;
            if is_exists {
                let err = AppError::BadRequest(String::from("用户名已存在"));
                return Err(err);
            }
            model.name = Set(name);
        }

        let real_name = dto.real_name.clone();
        if real_name.is_some() {
            model.real_name = Set(real_name.unwrap());
        }

        let nickname = dto.nickname.clone();
        if nickname.is_some() {
            model.nickname = Set(nickname.unwrap());
        }

        let user_types = dto.types_list.clone();
        if user_types.is_some() {
            let list: Vec<EditorTypeEnum> = user_types.unwrap();
            match platform {
                PlatformEnum::Open => {
                    let type_name = EditorTypeEnum::Member.string_value();
                    model.user_types = Set(type_name);
                }
                _ => {
                    let type_names = EditorTypeEnum::to_comma_str(&list);
                    model.user_types = Set(type_names);
                }
            };
        }

        if dto.gender.is_some() {
            let gender = dto.gender.clone().unwrap();
            match gender {
                GenderEnum::None => {}
                _ => {
                    model.gender = Set(gender.as_value());
                }
            }
        }

        let phone = dto.phone.clone();
        if phone.is_some() {
            let phone = phone.unwrap();
            let is_exists = Self::is_column_exist(
                id,
                UserColumn::Phone,
                sea_orm::Value::from(phone.to_owned()),
                db,
            )
            .await?;
            if is_exists {
                let err = AppError::BadRequest(String::from("手机号已存在"));
                return Err(err);
            }
            model.phone = Set(phone);
        }

        let avatar_path: Option<String> = dto.avatar_path.clone();
        if avatar_path.is_some() {
            model.avatar_path = Set(avatar_path.unwrap());
        }

        let email: Option<String> = dto.email.clone();
        if email.is_some() {
            let email = email.unwrap();
            let is_exists = Self::is_column_exist(
                id,
                UserColumn::Email,
                sea_orm::Value::from(email.to_owned()),
                db,
            )
            .await?;
            if is_exists {
                let err = AppError::BadRequest(String::from("邮箱已存在"));
                return Err(err);
            }
            model.email = Set(email);
        }

        let is_authed = match dto.is_authed {
            Some(flag) => flag,
            None => false,
        };
        model.is_authed = Set(is_authed);

        let is_enabled = match dto.is_enabled {
            Some(flag) => flag,
            None => true,
        };
        model.is_enabled = Set(is_enabled);

        let is_test = match dto.is_test {
            Some(flag) => flag,
            None => false,
        };
        model.is_test = Set(is_test);

        let time = time_utils::current_time();
        model.updated_at = Set(time);

        if is_create {
            model.created_at = Set(time);

            let no = dto.no.clone();
            let no = match no {
                Some(str) => str,
                None => {
                    let rand_str = random_utils::alpha_string(RAND_NO_LENGTH);
                    format!("U{}", rand_str)
                }
            };
            model.no = Set(no);

            let password = dto.password.clone();
            if password.is_some() {
                let password = password.unwrap();
                let password_str = password.as_str();
                let confirm_password = dto.confirm_password.clone();
                if confirm_password.is_none() {
                    let err = AppError::BadRequest(String::from("确认密码不能为空"));
                    return Err(err);
                } else if !confirm_password.unwrap().eq(password_str) {
                    let err = AppError::BadRequest(String::from("两次输入的密码不一致"));
                    return Err(err);
                }

                let salt = random_utils::alpha_string(RAND_SALT_LENGTH);
                let password = encrypt_password(salt.as_str(), password_str);
                model.salt = Set(salt);
                model.password = Set(password);
            }

            let data_source_id = match dto.data_source_id {
                Some(id) => id,
                None => 0,
            };
            model.data_source_id = Set(data_source_id);
        }

        let txn = db.begin().await?;

        let model = model.save(&txn).await?;
        let model = model.try_into_model()?;

        let opt_detail_origin = dto.detail.clone();
        let mut opt_detail: Option<DetailStoreDTO> = None;
        if opt_detail_origin.is_some() {
            let mut dto = opt_detail_origin.unwrap().clone();
            dto.user_id = Some(model.id);
            opt_detail = Some(dto);
        } else if is_create {
            opt_detail = Some(DetailStoreDTO {
                user_id: Some(model.id),
                ..Default::default()
            });
        }
        if let Some(dto) = opt_detail {
            Self::store_detail(&dto, state, &txn).await?;
        }

        // 提交事务
        txn.commit().await?;

        handle_ok(model)
    }

    async fn store_detail(
        dto: &DetailStoreDTO,
        state: &AppState,
        txn: &DatabaseTransaction,
    ) -> HandleResult<bool> {
        let user_id = dto.user_id.clone().unwrap_or(0);
        if user_id < 1 {
            return handle_ok(true);
        }
        let db = &state.db;
        let model = DetailEntity::find()
            .filter(DetailColumn::UserId.eq(user_id))
            .one(db)
            .await?;
        let mut is_create = false;
        let mut model: DetailActiveModel = match model {
            Some(model) => model.into(),
            None => {
                is_create = true;
                DetailActiveModel {
                    user_id: Set(user_id),
                    ..Default::default()
                }
            }
        };

        if dto.identity_no.is_some() {
            let identity_no = dto.identity_no.clone().unwrap();
            model.identity_no = Set(identity_no);
        }

        if dto.address.is_some() {
            let address = dto.address.clone().unwrap();
            model.address = Set(address);
        }

        if dto.born_on.is_some() {
            model.born_on = Set(dto.born_on.clone());
        } else {
            model.born_on = Set(None);
        }

        if dto.emotional.is_some() {
            let emotional = dto.emotional.clone().unwrap();
            model.emotional = Set(emotional);
        }

        if dto.graduated_from.is_some() {
            let graduated_from = dto.graduated_from.clone().unwrap();
            model.graduated_from = Set(graduated_from);
        }

        if dto.company_name.is_some() {
            let company_name = dto.company_name.clone().unwrap();
            model.company_name = Set(company_name);
        }

        if dto.staff_title.is_some() {
            let staff_title = dto.staff_title.clone().unwrap();
            model.staff_title = Set(staff_title);
        }

        if dto.introduction.is_some() {
            let introduction = dto.introduction.clone().unwrap();
            model.introduction = Set(introduction);
        }

        if dto.honor.is_some() {
            let honor = dto.honor.clone().unwrap();
            model.honor = Set(honor);
        }

        if dto.expertises.is_some() {
            let expertises = dto.expertises.clone().unwrap();
            model.expertises = Set(expertises);
        }

        let now = time_utils::current_time();
        if is_create {
            model.created_at = Set(now);
        }
        model.updated_at = Set(now);

        let _ = model.save(txn).await?;

        handle_ok(true)
    }

    /// 检查字段值是否唯一
    async fn is_column_exist(
        id: i64,
        column: UserColumn,
        value: sea_orm::Value,
        db: &DatabaseConnection,
    ) -> HandleResult<bool> {
        let count = Self::scope_active_query()
            .select_only()
            .column(UserColumn::Id)
            .filter(column.eq(value))
            .filter(UserColumn::Id.ne(id))
            .count(db)
            .await?;

        handle_ok(count > 0)
    }

    /// 表单选项
    pub fn form_options() -> HandleResult<UserFormOptionVO> {
        let genders = GenderEnum::to_option_list();
        let types = EditorTypeEnum::to_option_list();

        let vo = UserFormOptionVO {
            genders: Some(genders),
            types: Some(types),
        };

        handle_ok(vo)
    }

    /// 检查字段值是否唯一
    pub async fn field_unique(dto: &FieldValueUniqueDTO, state: &AppState) -> HandleResult<bool> {
        let id = dto.skip_id;
        let db = &state.db;

        let field_name = dto.field_name.to_owned();
        let column = match field_name.to_lowercase().as_str() {
            "name" | "username" => UserColumn::Name,
            "email" => UserColumn::Email,
            "mobile" | "phone" => UserColumn::Phone,
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
            "is_enabled" => UserColumn::IsEnabled,
            "is_authed" => UserColumn::IsAuthed,
            "is_test" => UserColumn::IsTest,
            _ => {
                let err = AppError::BadRequest(String::from("无效的字段"));
                return Err(err);
            }
        };

        let _update_rows_count = UserEntity::update_many()
            .col_expr(column, Expr::value(dto.field_value))
            .filter(UserColumn::Id.eq(id))
            .exec(db)
            .await?;

        handle_ok(true)
    }

    /// 修改登录密码
    pub async fn update_password(
        dto: &UserUpdatePasswordDTO,
        state: &AppState,
    ) -> HandleResult<bool> {
        let id = dto.id;
        if id < 1 {
            let err = AppError::BadRequest(String::from("无效的用户ID"));
            return Err(err);
        }

        let db = &state.db;
        let model = Self::fetch_by_id(id, state).await?;
        if dto.current_password.is_some() {
            let current_password = dto.current_password.clone().unwrap();
            let salt = model.salt.clone();
            let md5_password = encrypt_password(salt.as_str(), &current_password.as_str());
            if md5_password.ne(model.password.as_str()) {
                let err = AppError::BadRequest(String::from("当前密码不正确"));
                return Err(err);
            }
        }

        let new_password = dto.new_password.clone();
        let confirm_password = dto.confirm_password.clone();
        if confirm_password.ne(new_password.as_str()) {
            let err = AppError::BadRequest(String::from("两次输入的密码不一致"));
            return Err(err);
        }

        let salt = random_utils::alpha_string(RAND_SALT_LENGTH);
        let password = encrypt_password(salt.as_str(), new_password.as_str());
        // if password.eq(model.password.as_str()) {
        //     let err = AppError::BadRequest(String::from("新密码不能与旧密码相同"));
        //     return Err(err);
        // }

        let mut active: UserActiveModel = model.into();
        active.password = Set(password.to_owned());
        active.salt = Set(salt.to_owned());
        active.updated_at = Set(time_utils::current_time());
        let _ = active.update(db).await?;

        handle_ok(true)
    }

    pub async fn view(
        platform: &PlatformEnum,
        dto: &UserViewDTO,
        state: &AppState,
    ) -> HandleResult<UserItemVO> {
        let id = dto.id;
        if id < 1 {
            let err = AppError::BadRequest(String::from("无效的用户ID"));
            return Err(err);
        }

        let db = &state.db;
        let model = Self::fetch_by_id(id, state).await?;
        if *platform == PlatformEnum::Open {
            if model.is_test {
                let err = AppError::BadRequest(String::from("无效的用户ID"));
                return Err(err);
            } else if !model.is_enabled {
                let err = AppError::BadRequest(String::from("无效的用户ID"));
                return Err(err);
            }
        }

        let mut vo: UserItemVO = model.into();
        if let Some(load_models) = dto.load_models.clone() {
            for enums in load_models {
                match enums {
                    UserLoadEnum::Editor => {
                        vo.editor = EditorService::load_by_id(vo.editor_id, state).await?;
                    }
                    UserLoadEnum::Detail => {
                        let detail = DetailEntity::find()
                            .filter(DetailColumn::UserId.eq(id))
                            .one(db)
                            .await?;
                        if let Some(detail) = detail {
                            vo.detail = Some(detail.into());
                        }
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
        dto: &UserQueryDTO,
        state: &AppState,
    ) -> HandleResult<PaginateResultVO<UserItemVO>> {
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
        let mut list: Vec<UserItemVO> = Vec::with_capacity(len);
        let mut editor_ids: Vec<i64> = Vec::with_capacity(len);
        for model in models.iter() {
            editor_ids.push(model.editor_id);
            let vo: UserItemVO = model.into();
            list.push(vo);
        }

        if let Some(load_models) = dto.load_models.clone() {
            for enums in load_models {
                match enums {
                    UserLoadEnum::Editor => {
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

        println!("lenth: {}", editor_ids.len());
        let vo = PaginateResultVO {
            page_size,
            current_page: page,
            total: total.number_of_items,
            list,
        };

        // let editor_ids = models
        //     .into_iter()
        //     .map(|model| model.editor_id)
        //     .collect::<Vec<i64>>();
        handle_ok(vo)
    }

    async fn query_builder(
        platform: &PlatformEnum,
        dto: &UserQueryDTO,
    ) -> HandleResult<Select<UserEntity>> {
        let mut query = Self::scope_active_query();
        query = query.order_by_desc(UserColumn::Id);

        match platform {
            PlatformEnum::Open => {
                query = query.filter(UserColumn::IsTest.eq(false));
            }
            _ => {
                query = query.filter(UserColumn::IsTest.eq(false));
            }
        }

        if let Some(keyword) = dto.keyword.clone() {
            let condition = Condition::any()
                .add(UserColumn::Name.contains(&keyword))
                .add(UserColumn::Nickname.contains(&keyword))
                .add(UserColumn::Phone.contains(&keyword))
                .add(UserColumn::Email.contains(&keyword));
            query = query.filter(condition);
        } else {
            if let Some(phone) = dto.phone.clone() {
                query = query.filter(UserColumn::Phone.eq(phone));
            }

            if let Some(email) = dto.email.clone() {
                query = query.filter(UserColumn::Email.eq(email));
            }
        }

        if let Some(enabled) = dto.is_enabled {
            query = query.filter(UserColumn::IsEnabled.eq(enabled));
        }

        if let Some(authed) = dto.is_authed {
            query = query.filter(UserColumn::IsAuthed.eq(authed));
        }

        if let Some(test) = dto.is_test {
            query = query.filter(UserColumn::IsTest.eq(test));
        }

        if let Some(gender) = dto.gender.clone() {
            match gender {
                GenderEnum::Male | GenderEnum::Female | GenderEnum::Unknown => {
                    let value = gender.as_value();
                    query = query.filter(UserColumn::Gender.eq(value));
                }
                _ => {}
            }
        }

        if let Some(time) = dto.login_start_time.clone() {
            query = query.filter(UserColumn::LastLoginAt.gte(time));
        }

        if let Some(time) = dto.login_end_time.clone() {
            query = query.filter(UserColumn::LastLoginAt.lte(time));
        }

        if let Some(time) = dto.created_start_time.clone() {
            query = query.filter(UserColumn::CreatedAt.gte(time));
        }

        if let Some(time) = dto.created_end_time.clone() {
            query = query.filter(UserColumn::CreatedAt.lte(time));
        }

        handle_ok(query)
    }

    fn scope_active_query() -> Select<UserEntity> {
        UserEntity::find().filter(UserColumn::IsDeleted.eq(false))
    }

    async fn fetch_by_id(id: i64, state: &AppState) -> HandleResult<UserModel> {
        let db = &state.db;
        let model = Self::scope_active_query()
            .filter(UserColumn::Id.eq(id))
            .one(db)
            .await?
            .ok_or_else(|| AppError::BadRequest(String::from("无效的用户ID")))?;

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
            let mut model: UserActiveModel = model.into();
            model.is_deleted = Set(true);
            let now = time_utils::current_time();
            model.deleted_at = Set(Some(now));
            let _ = model.save(db).await?;
        }

        handle_ok(())
    }
}
