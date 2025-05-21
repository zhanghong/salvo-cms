use salvo::oapi::ToSchema;
use serde::{Deserialize, Serialize};

use crate::consts::enum_consts::*;
use crate::error::AppError;

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, ToSchema)]
#[salvo(schema(name = "Core::Enum::ErrorEnum"))]
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
            ErrorEnum::VersionNoInvalid => ERROR_VERSION_INVALID_MESSAGE,
            ErrorEnum::NameExists => ERROR_NAME_EXISTS_MESSAGE,
            ErrorEnum::TitleExists => ERROR_TITLE_EXISTS_MESSAGE,
            ErrorEnum::FieldInvalid => ERROR_FIELD_INVALID_MESSAGE,
            ErrorEnum::ParamIdInvalid => ERROR_PARAM_ID_INVALID_MESSAGE,
            ErrorEnum::UpdateFieldInvalid => ERROR_UPDATE_FIELD_INVALID_MESSAGE,
            ErrorEnum::RecordNotFound => ERROR_RECORD_NOT_FOUND_MESSAGE,
            ErrorEnum::NoPermissionDelete => ERROR_NO_PERMISSION_DELETE_MESSAGE,
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::error::AppError;

    // ----------------------------
    // 测试 ErrorEnum::message()
    // ----------------------------
    #[test]
    fn test_message_returns_correct_strings() {
        assert_eq!(
            ErrorEnum::VersionNoInvalid.message(),
            ERROR_VERSION_INVALID_MESSAGE
        );
        assert_eq!(ErrorEnum::NameExists.message(), ERROR_NAME_EXISTS_MESSAGE);
        assert_eq!(ErrorEnum::TitleExists.message(), ERROR_TITLE_EXISTS_MESSAGE);
        assert_eq!(
            ErrorEnum::FieldInvalid.message(),
            ERROR_FIELD_INVALID_MESSAGE
        );
        assert_eq!(
            ErrorEnum::ParamIdInvalid.message(),
            ERROR_PARAM_ID_INVALID_MESSAGE
        );
        assert_eq!(
            ErrorEnum::UpdateFieldInvalid.message(),
            ERROR_UPDATE_FIELD_INVALID_MESSAGE
        );
        assert_eq!(
            ErrorEnum::RecordNotFound.message(),
            ERROR_RECORD_NOT_FOUND_MESSAGE
        );
        assert_eq!(
            ErrorEnum::NoPermissionDelete.message(),
            ERROR_NO_PERMISSION_DELETE_MESSAGE
        );
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
            assert_eq!(msg, ERROR_NAME_EXISTS_MESSAGE);
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
            assert_eq!(msg, ERROR_TITLE_EXISTS_MESSAGE);
        } else {
            panic!("Expected AppError::BadRequest");
        }
    }
}
