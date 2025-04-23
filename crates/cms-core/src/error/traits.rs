// 错误trait
pub trait ErrorTrait {
    fn code(&self) -> i64;
    fn message(&self) -> String;
}
