use chrono::NaiveDate;
use salvo::oapi::ToSchema;
use serde::{Deserialize, Serialize};
use validator::{Validate, ValidationError};

use cms_core::utils::validate;

// // ------------------------------------
// // 字段验证方法
// // ------------------------------------
fn validate_field_identity_no(ptr: &&String) -> Result<(), ValidationError> {
    let string = (*ptr).clone();
    let str = string.as_str();
    validate::string_length(str, false, 18, 18)
}

fn validate_big_integer_present(num: i64) -> Result<(), ValidationError> {
    validate::numeric_equal_or_greater_than(Some(num), 0)
}

fn validate_field_address(ptr: &&String) -> Result<(), ValidationError> {
    let string = (*ptr).clone();
    let str = string.as_str();
    validate::string_max_length(str, 150)
}

fn validate_field_emotional(ptr: &&String) -> Result<(), ValidationError> {
    let string = (*ptr).clone();
    let str = string.as_str();
    validate::string_max_length(str, 50)
}

fn validate_field_graduated_from(ptr: &&String) -> Result<(), ValidationError> {
    let string = (*ptr).clone();
    let str = string.as_str();
    validate::string_max_length(str, 80)
}

fn validate_field_company_name(ptr: &&String) -> Result<(), ValidationError> {
    let string = (*ptr).clone();
    let str = string.as_str();
    validate::string_max_length(str, 100)
}

fn validate_field_staff_title(ptr: &&String) -> Result<(), ValidationError> {
    let string = (*ptr).clone();
    let str = string.as_str();
    validate::string_max_length(str, 50)
}

fn validate_field_text_content(ptr: &&String) -> Result<(), ValidationError> {
    let string = (*ptr).clone();
    let str = string.as_str();
    validate::string_max_length(str, 500)
}

// ------------------------------------
// 创建/更新用户详情
// ------------------------------------
// Service 层创建/更新用户使用的结构体
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Default, Validate, ToSchema)]
pub struct DetailStoreForm {
    /// 身份证号
    #[validate(custom(function = "validate_field_identity_no", message = "格式无效"))]
    pub identity_no: Option<String>,

    /// 所在省
    #[validate(custom(function = "validate_big_integer_present", message = "输入数据无效"))]
    pub province_id: Option<i64>,

    /// 所在城市
    #[validate(custom(function = "validate_big_integer_present", message = "输入数据无效"))]
    pub city_id: Option<i64>,

    /// 所在区县
    #[validate(custom(function = "validate_big_integer_present", message = "输入数据无效"))]
    pub district_id: Option<i64>,

    /// 详情地址
    #[validate(custom(function = "validate_field_address", message = "不能超过150个字符"))]
    pub address: Option<String>,

    /// 出生日期
    pub born_on: Option<NaiveDate>,

    /// 情感状态
    #[validate(custom(function = "validate_field_emotional", message = "不能超过50个字符"))]
    pub emotional: Option<String>,

    /// 毕业院校
    #[validate(custom(
        function = "validate_field_graduated_from",
        message = "不能超过80个字符"
    ))]
    pub graduated_from: Option<String>,

    /// 公司名称
    #[validate(custom(
        function = "validate_field_company_name",
        message = "不能超过100个字符"
    ))]
    pub company_name: Option<String>,

    /// 职位名称
    #[validate(custom(function = "validate_field_staff_title", message = "不能超过50个字符"))]
    pub staff_title: Option<String>,

    /// 个人简介
    #[validate(custom(
        function = "validate_field_text_content",
        message = "不能超过500个字符"
    ))]
    pub introduction: Option<String>,

    /// 荣誉奖项
    #[validate(custom(
        function = "validate_field_text_content",
        message = "不能超过500个字符"
    ))]
    pub honor: Option<String>,

    /// 擅长领域
    #[validate(custom(
        function = "validate_field_text_content",
        message = "不能超过500个字符"
    ))]
    pub expertises: Option<String>,
}
