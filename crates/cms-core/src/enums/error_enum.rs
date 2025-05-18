use salvo::oapi::ToSchema;
use serde::{Deserialize, Serialize};

use crate::error::AppError;

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

impl ErrorEnum {
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

    pub fn into_app_error(self) -> AppError
    where
        Self: Sized,
    {
        match self {
            ErrorEnum::RecordNotFound => AppError::NotFound(self.message()),
            ErrorEnum::NoPermissionDelete => AppError::Forbidden,
            _ => AppError::BadRequest(self.message()),
        }
    }
}

impl Into<AppError> for ErrorEnum {
    fn into(self) -> AppError {
        self.into_app_error()
    }
}

mod tests {
    use super::*;
    use crate::error::AppError;

    // ----------------------------
    // 测试 ErrorEnum::message()
    // ----------------------------
    #[test]
    fn test_message_returns_correct_strings() {
        assert_eq!(ErrorEnum::VersionNoInvalid.message(), "版本号错误");
        assert_eq!(ErrorEnum::NameExists.message(), "名称已存在");
        assert_eq!(ErrorEnum::TitleExists.message(), "标题已存在");
        assert_eq!(ErrorEnum::FieldInvalid.message(), "无效的字段");
        assert_eq!(ErrorEnum::ParamIdInvalid.message(), "参数ID错误");
        assert_eq!(ErrorEnum::UpdateFieldInvalid.message(), "更新字段错误");
        assert_eq!(ErrorEnum::RecordNotFound.message(), "访问记录不存在");
        assert_eq!(ErrorEnum::NoPermissionDelete.message(), "无权限删除");
    }

    // ----------------------------
    // 测试 ErrorEnum::into_app_error()
    // ----------------------------
    #[test]
    fn test_into_app_error_for_record_not_found() {
        let err = ErrorEnum::RecordNotFound.into_app_error();
        if let AppError::NotFound(msg) = err {
            assert_eq!(msg, "访问记录不存在");
        } else {
            panic!("Expected AppError::NotFound");
        }
    }

    #[test]
    fn test_into_app_error_for_no_permission_delete() {
        let err = ErrorEnum::NoPermissionDelete.into_app_error();
        assert_eq!(err, AppError::Forbidden);
    }

    #[test]
    fn test_into_app_error_for_other_errors() {
        let err = ErrorEnum::NameExists.into_app_error();
        if let AppError::BadRequest(msg) = err {
            assert_eq!(msg, "名称已存在");
        } else {
            panic!("Expected AppError::BadRequest");
        }
    }

    // ----------------------------
    // 测试 Into<AppError> trait
    // ----------------------------
    #[test]
    fn test_into_app_error_trait() {
        let err: AppError = ErrorEnum::TitleExists.into();
        if let AppError::BadRequest(msg) = err {
            assert_eq!(msg, "标题已存在");
        } else {
            panic!("Expected AppError::BadRequest");
        }
    }
}