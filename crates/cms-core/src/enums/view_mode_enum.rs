use salvo::oapi::ToSchema;
use serde::{Deserialize, Serialize};

use super::PlatformEnum;

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, ToSchema)]
pub enum ViewModeEnum {
    ManagerList,
    ManagerDetail,
    OpenList,
    OpenDetail,
}

impl ViewModeEnum {
    pub fn platform_to_list_mode(platform: &PlatformEnum) -> Self {
        match *platform {
            PlatformEnum::Open => Self::OpenList,
            _ => Self::ManagerList,
        }
    }

    pub fn platform_to_detail_mode(platform: &PlatformEnum) -> Self {
        match *platform {
            PlatformEnum::Open => Self::OpenDetail,
            _ => Self::ManagerDetail,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::enums::PlatformEnum;

    #[test]
    fn test_platform_to_list_mode_open() {
        let result = ViewModeEnum::platform_to_list_mode(&PlatformEnum::Open);
        assert_eq!(result, ViewModeEnum::OpenList);
    }

    #[test]
    fn test_platform_to_list_mode_manager() {
        let result = ViewModeEnum::platform_to_list_mode(&PlatformEnum::Manager);
        assert_eq!(result, ViewModeEnum::ManagerList);
    }

    #[test]
    fn test_platform_to_list_mode_admin() {
        let result = ViewModeEnum::platform_to_list_mode(&PlatformEnum::System);
        assert_eq!(result, ViewModeEnum::ManagerList);
    }

    #[test]
    fn test_platform_to_detail_mode_open() {
        let result = ViewModeEnum::platform_to_detail_mode(&PlatformEnum::Open);
        assert_eq!(result, ViewModeEnum::OpenDetail);
    }

    #[test]
    fn test_platform_to_detail_mode_manager() {
        let result = ViewModeEnum::platform_to_detail_mode(&PlatformEnum::Manager);
        assert_eq!(result, ViewModeEnum::ManagerDetail);
    }

    #[test]
    fn test_platform_to_detail_mode_admin() {
        let result = ViewModeEnum::platform_to_detail_mode(&PlatformEnum::System);
        assert_eq!(result, ViewModeEnum::ManagerDetail);
    }
}
