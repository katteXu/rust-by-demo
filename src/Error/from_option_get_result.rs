use std::num::ParseIntError;

fn double_first(vec: Vec<&str>) -> Option<Result<i32, ParseIntError>> {
    // let first = vec.first().unwrap();
    // 2 * first.parse::<i32>().unwrap()

    vec.first().map(|first| first.parse::<i32>().map(|v| 2 * v))
}

fn double_first2(vec: Vec<&str>) -> Result<Option<i32>, ParseIntError> {
    let opt = vec
        .first()
        .map(|first| first.parse::<i32>().map(|first| 2 * first));

    opt.map_or(Ok(None), |r| r.map(Some))
}

fn main() {
    let vec0 = vec!["11", "2"];
    let vec1: Vec<&str> = vec![];
    let vec2 = vec!["abc"];
    let result0 = double_first(vec0);
    let result1 = double_first(vec1);
    let result2 = double_first(vec2);
    println!("{:?}", result0);
    println!("{:?}", result1);
    println!("{:?}", result2);
}
