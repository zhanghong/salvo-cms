use serde::{Deserialize, Serialize};

use crate::domain::form::DetailStoreForm;

// ------------------------------------
// 创建/更新用户详情
// ------------------------------------
// Service 层创建/更新用户使用的结构体
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Default)]
pub struct DetailStoreDTO {
    /// 用户ID
    pub user_id: Option<i64>,

    /// 身份证号
    pub identity_no: Option<String>,

    /// 所在省
    pub province_id: Option<i64>,

    /// 所在城市
    pub city_id: Option<i64>,

    /// 所在区县
    pub district_id: Option<i64>,

    /// 详情地址
    pub address: Option<String>,

    /// 情感状态
    pub emotional: Option<String>,

    /// 毕业院校
    pub graduated_from: Option<String>,

    /// 公司名称
    pub company_name: Option<String>,

    /// 职位名称
    pub staff_title: Option<String>,

    /// 个人简介
    pub introduction: Option<String>,

    /// 荣誉奖项
    pub honor: Option<String>,

    /// 擅长领域
    pub expertises: Option<String>,
}

impl From<DetailStoreForm> for DetailStoreDTO {
    fn from(form: DetailStoreForm) -> Self {
        Self {
            identity_no: form.identity_no,
            province_id: form.province_id,
            city_id: form.city_id,
            district_id: form.district_id,
            address: form.address,
            emotional: form.emotional,
            graduated_from: form.graduated_from,
            company_name: form.company_name,
            staff_title: form.staff_title,
            introduction: form.introduction,
            honor: form.honor,
            expertises: form.expertises,
            ..Default::default()
        }
    }
}
