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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_page_no_default() {
        assert_eq!(page_no_default(), 1);
    }

    #[test]
    fn test_page_size_min() {
        assert_eq!(page_size_min(), 10);
    }

    #[test]
    fn test_page_size_max() {
        assert_eq!(page_size_max(), 50);
    }

    #[test]
    fn test_page_size_default() {
        assert_eq!(page_size_default(), 20);
    }

    #[test]
    fn test_set_value_within_range_with_none() {
        assert_eq!(set_value_within_range(None, 10, 50, 20), 20);
    }

    #[test]
    fn test_set_value_within_range_less_than_min() {
        assert_eq!(set_value_within_range(Some(5), 10, 50, 20), 10);
    }

    #[test]
    fn test_set_value_within_range_greater_than_max() {
        assert_eq!(set_value_within_range(Some(100), 10, 50, 20), 50);
    }

    #[test]
    fn test_set_value_within_range_in_range() {
        assert_eq!(set_value_within_range(Some(30), 10, 50, 20), 30);
    }

    #[test]
    fn test_page_no_set_with_none() {
        assert_eq!(page_no_set(None), 1);
    }

    #[test]
    fn test_page_no_set_less_than_min() {
        assert_eq!(page_no_set(Some(0)), 1);
    }

    #[test]
    fn test_page_no_set_valid_value() {
        assert_eq!(page_no_set(Some(100)), 100);
    }

    #[test]
    fn test_page_size_set_with_none() {
        assert_eq!(page_size_set(None), 20);
    }

    #[test]
    fn test_page_size_set_less_than_min() {
        assert_eq!(page_size_set(Some(5)), 10);
    }

    #[test]
    fn test_page_size_set_greater_than_max() {
        assert_eq!(page_size_set(Some(100)), 50);
    }

    #[test]
    fn test_page_size_set_valid_value() {
        assert_eq!(page_size_set(Some(25)), 25);
    }
}
