pub fn page_no_default() -> u64 {
    1
}

pub fn page_no_set(opt: Option<u64>) -> u64 {
    let min = page_no_default();
    match opt {
        Some(num) => {
            if num < min {
                min
            } else {
                num
            }
        }
        _ => page_no_default(),
    }
}

pub fn page_size_min() -> u64 {
    10
}

pub fn page_size_max() -> u64 {
    50
}

pub fn page_size_default() -> u64 {
    20
}

pub fn page_size_set(opt: Option<u64>) -> u64 {
    let min = page_size_min();
    let max = page_size_max();
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
        _ => page_size_default(),
    }
}
