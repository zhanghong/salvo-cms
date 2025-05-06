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
