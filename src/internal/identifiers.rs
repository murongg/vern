#[derive(Debug)]
pub enum CompareType {
    LARGER = 1,
    SMALLER = -1,
    EQUAL = 0,
}


pub fn compare_identifiers_with_int(a: i32, b: i32) -> i8 {
    if a == b {
        0
    } else if a > b {
        1
    } else {
        -1
    }
}

pub fn compare_identifiers(a: i32, b: i32) -> CompareType {
    let compare = compare_identifiers_with_int(a, b);
    if compare == 0 {
        CompareType::EQUAL
    } else if compare == 1 {
        CompareType::LARGER
    } else {
        CompareType::SMALLER
    }
}
