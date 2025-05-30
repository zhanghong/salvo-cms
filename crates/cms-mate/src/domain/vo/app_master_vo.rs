use redis_macros::{FromRedisValue, ToRedisArgs};
use salvo::oapi::{
    Array, BasicType, Object, Ref, RefOr, Schema, SchemaType, ToSchema, schema::OneOf,
};
use serde::{Deserialize, Serialize};

use cms_core::{domain::vo::EditorLoadVO, enums::ViewModeEnum};

use crate::domain::entity::app::Model;

// fn custom_type() -> OneOf {
//     // OneOf::new().schema_type(SchemaType::Basic(BasicType::Null))
//     // .item(Schema::Array(Array::new().items(RefOr::Type(
//     //     Schema::Object(Object::new().property("element", RefOr::Ref(Ref::new("#/test")))),
//     // ))))
//     // .item(Schema::Array(Array::new().items(RefOr::Type(
//     //     Schema::Object(Object::new().property("foobar", RefOr::Ref(Ref::new("#/foobar")))),
//     // ))))

//     OneOf::new()
//         .item(Schema::Object(
//             Object::new().schema_type(SchemaType::Basic(BasicType::Null)),
//         ))
//         .item(Schema::Object(
//             Object::new().schema_type(SchemaType::Basic(BasicType::Null)),
//         ))
//         .item(Ref::from_schema_name("EditorLoadVO"))
//         .item(Schema::Object(
//             Object::new().property("foobar", RefOr::Ref(Ref::new("#/foobar"))),
//         ))
//         .item(Schema::Array(Array::new().items(RefOr::Type(
//             Schema::Object(Object::new().name("EditorLoadVO".to_string())),
//         ))))
// }

/// App 主 VO
#[derive(
    Debug, Clone, PartialEq, Default, Deserialize, Serialize, ToSchema, FromRedisValue, ToRedisArgs,
)]
#[salvo(schema(name = "Mate/App/AppMasterVO"))]
pub struct AppMasterVO {
    /// 主键
    #[salvo(parameter(required = true, nullable = false, minimum = 1, example = 1))]
    pub id: i64,

    /// 编辑用户类型
    #[serde(skip_serializing)]
    #[salvo(parameter(required = false, nullable = false, max_length = 10, example = "admin"))]
    pub editor_type: String,

    /// 编辑用户ID
    #[serde(skip_serializing)]
    #[salvo(parameter(required = false, nullable = false, minimum = 0, example = 1))]
    pub editor_id: i64,

    /// 名称
    #[salvo(parameter(
        required = true,
        nullable = false,
        max_length = 30,
        example = "product"
    ))]
    pub name: String,

    /// 标题
    #[salvo(parameter(required = true, nullable = false, max_length = 30, example = "商品"))]
    pub title: String,

    /// 描述
    #[salvo(parameter(
        required = true,
        nullable = false,
        max_length = 200,
        example = "商品描述..."
    ))]
    pub description: String,

    /// 图标
    #[salvo(schema(
        required = true,
        nullable = false,
        max_length = 30,
        pattern = r"^[a-zA-Z0-9_-]+$",
        example = "icon-product"
    ))]
    pub icon: String,

    /// 版本号
    #[serde(skip_serializing_if = "Option::is_none")]
    #[salvo(schema(required = false, nullable = false, value_type = i32, minimum = 1, example = 3))]
    pub version_no: Option<i32>,

    /// 排序编号
    #[salvo(schema(required = true, nullable = false, value_type = i16, minimum = 0, maximum = 9999, example = 80, default = 99))]
    pub sort: i16,

    /// 是否启用
    #[salvo(schema(required = false, nullable = true, value_type = bool, example = true, default = true))]
    pub is_enabled: bool,

    /// 是否可以更新
    #[serde(skip_serializing_if = "Option::is_none")]
    #[salvo(schema(required = false, nullable = false, value_type = bool, example = true, default = true))]
    pub can_update: Option<bool>,

    /// 是否可以删除
    #[serde(skip_serializing_if = "Option::is_none")]
    #[salvo(schema(required = false, nullable = false, value_type = bool, example = true, default = true))]
    pub can_delete: Option<bool>,

    /// 创建时间
    #[serde(skip_serializing_if = "Option::is_none")]
    #[salvo(schema(required = false, nullable = false, value_type = String, format = "yyyy-mm-dd HH:MM:SS", example = "2023-08-10 10:00:00"))]
    pub created_time: Option<String>,

    /// 更新时间
    #[serde(skip_serializing_if = "Option::is_none")]
    #[salvo(schema(
        required = false,
        nullable = true,
        inline = true,
        value_type = EditorLoadVO,
        format = "yyyy-mm-dd HH:MM:SS",
        example = "2023-08-10 10:00:00"
    ))]
    pub updated_time: Option<String>,

    /// 编辑用户
    #[serde(skip_serializing_if = "Option::is_none")]
    #[salvo(schema(required = false, nullable = true, default = "SchemaType::any"))]
    pub editor: Option<EditorLoadVO>,
}

impl AppMasterVO {
    pub fn mode_into(view_enum: &ViewModeEnum, model: &Model) -> Self {
        let mut vo = Self {
            id: model.id,
            name: model.name.to_owned(),
            title: model.title.to_owned(),
            description: model.description.to_owned(),
            icon: model.icon.to_owned(),
            sort: model.sort,
            is_enabled: model.is_enabled,
            ..Default::default()
        };

        match *view_enum {
            ViewModeEnum::ManagerDetail | ViewModeEnum::ManagerList => {
                vo.editor_type = model.editor_type.to_owned();
                vo.editor_id = model.editor_id;
                vo.version_no = model.version_no;
                vo.created_time = model.created_time().clone();
                vo.updated_time = model.updated_time().clone();
            }
            _ => {}
        }

        vo
    }
}
