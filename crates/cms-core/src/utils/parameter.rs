/// 默认页码
pub fn page_no_default() -> u64 {
    1
}

/// 最小页面大小
pub fn page_size_min() -> u64 {
    10
}

/// 最大页面大小
pub fn page_size_max() -> u64 {
    50
}

/// 默认页面大小
pub fn page_size_default() -> u64 {
    20
}

/// 通用设置函数，用于限制值在指定范围内
fn set_value_within_range(opt: Option<u64>, min: u64, max: u64, default: u64) -> u64 {
    match opt {
        Some(num) => {
            if num < min {
                min
            } else if num > max {
                max
            } else {
                num
            }
        }
        _ => default,
    }
}

/// 设置页码，默认为1，最小值为1
pub fn page_no_set(opt: Option<u64>) -> u64 {
    let min = page_no_default();
    set_value_within_range(opt, min, u64::MAX, min)
}

/// 设置页面大小，默认为20，范围为10到50
pub fn page_size_set(opt: Option<u64>) -> u64 {
    let min = page_size_min();
    let max = page_size_max();
    let default = page_size_default();
    set_value_within_range(opt, min, max, default)
}
