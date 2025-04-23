// 引入必要的trait
use salvo::oapi::ToSchema;
use serde::{Deserialize, Serialize};

use crate::error::ErrorTrait;

const MSG_VERSION_INVALID: &str = "版本号错误";
const MSG_NAME_EXISTS: &str = "名称已存在";
const MSG_TITLE_EXISTS_MSG: &str = "标题已存在";
const MSG_FIELD_INVALID: &str = "无效的字段";
const MSG_PARAM_ID_INVALID: &str = "参数ID错误";
const MSG_UPDATE_FIELD_INVALID: &str = "更新字段错误";
const MSG_RECORD_NOT_FOUND: &str = "访问记录不存在";
const MSG_NO_PERMISSION_DELETE: &str = "无权限删除";

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, ToSchema)]
pub enum ErrorEnum {
    VersionNoInvalid,
    NameExists,
    TitleExists,
    FieldInvalid,
    ParamIdInvalid,
    UpdateFieldInvalid,
    RecordNotFound,
    NoPermissionDelete,
}

impl ErrorTrait for ErrorEnum {
    fn code(&self) -> i64 {
        match self {
            ErrorEnum::VersionNoInvalid => 10001,
            ErrorEnum::NameExists => 10002,
            ErrorEnum::TitleExists => 10003,
            ErrorEnum::FieldInvalid => 10004,
            ErrorEnum::ParamIdInvalid => 10005,
            ErrorEnum::UpdateFieldInvalid => 10006,
            ErrorEnum::RecordNotFound => 10007,
            ErrorEnum::NoPermissionDelete => 10008,
        }
    }
    fn message(&self) -> String {
        let str = match self {
            ErrorEnum::VersionNoInvalid => MSG_VERSION_INVALID,
            ErrorEnum::NameExists => MSG_NAME_EXISTS,
            ErrorEnum::TitleExists => MSG_TITLE_EXISTS_MSG,
            ErrorEnum::FieldInvalid => MSG_FIELD_INVALID,
            ErrorEnum::ParamIdInvalid => MSG_PARAM_ID_INVALID,
            ErrorEnum::UpdateFieldInvalid => MSG_UPDATE_FIELD_INVALID,
            ErrorEnum::RecordNotFound => MSG_RECORD_NOT_FOUND,
            ErrorEnum::NoPermissionDelete => MSG_NO_PERMISSION_DELETE,
        };
        str.to_string()
    }
}
