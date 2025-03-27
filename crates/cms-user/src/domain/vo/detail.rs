use salvo::oapi::ToSchema;
use serde::{Deserialize, Serialize};

use crate::domain::entity::detail::Model;

// ------------------------------------
// 用户详情
// ------------------------------------
// Service 层创建/更新用户使用的结构体
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Default, ToSchema)]
pub struct DetailVO {
    /// 主键
    pub id: i64,

    pub editor_type: String,

    pub editor_id: i64,

    /// 身份证号
    pub identity_no: String,

    /// 所在省
    pub province_id: i64,

    /// 所在城市
    pub city_id: i64,

    /// 所在区县
    pub district_id: i64,

    /// 详情地址
    pub address: String,

    /// 出生日期
    pub born_date: Option<String>,

    /// 情感状态
    pub emotional: String,

    /// 毕业院校
    pub graduated_from: String,

    /// 公司名称
    pub company_name: String,

    /// 职位名称
    pub staff_title: String,

    /// 个人简介
    pub introduction: String,

    /// 荣誉奖项
    pub honor: String,

    /// 擅长领域
    pub expertises: String,
}

impl From<Model> for DetailVO {
    fn from(model: Model) -> Self {
        let born_date = model.born_date();
        Self {
            id: model.id,
            editor_type: model.editor_type.to_owned(),
            editor_id: model.editor_id,
            identity_no: model.identity_no.to_owned(),
            province_id: model.province_id,
            city_id: model.city_id,
            district_id: model.district_id,
            address: model.address.to_owned(),
            born_date,
            emotional: model.emotional.to_owned(),
            graduated_from: model.graduated_from.to_owned(),
            company_name: model.company_name.to_owned(),
            staff_title: model.staff_title.to_owned(),
            introduction: model.introduction.to_owned(),
            honor: model.honor.to_owned(),
            expertises: model.expertises.to_owned(),
            ..Default::default()
        }
    }
}
