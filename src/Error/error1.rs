use std::fmt;

use std::error::Error;

/**
 * 定义一个错误类型
 * 1. 用同一个类型代表了多种错误
 * 2. 向用户提供了清楚的错误信息
 * 3. 能够容易地与其他类型比较
 *      好的例子：Err(EmptyVec)
 *      坏的例子：Err("Please use a vector with at least one element".to_owned())
 * 4. 能够容纳错误的具体信息
 *      好的例子：Err(BadChar(c, position))
 *      坏的例子：Err("+ cannot be used here".to_owned())
 * 5. 能够与其他错误很好地整合
 */

#[derive(Debug, Clone)]
struct DoubleError;
impl fmt::Display for DoubleError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "invalid first item to double")
    }
}

impl Error for DoubleError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        None
    }
}
type Result<T> = std::result::Result<T, DoubleError>;

fn double_first(vec: Vec<&str>) -> Result<i32> {
    vec.first()
        .ok_or(DoubleError)
        .and_then(|first| first.parse::<i32>().map_err(|_| DoubleError).map(|v| v * 2))
}

fn print(result: Result<i32>) {
    match result {
        Ok(n) => println!("The first doubled is {}", n),
        Err(e) => println!("Error: {}", e),
    }
}

fn main() {
    let vec0 = vec!["11", "2"];
    let vec1: Vec<&str> = vec![];
    let vec2 = vec!["abc"];
    let result0 = double_first(vec0);
    let result1 = double_first(vec1);
    let result2 = double_first(vec2);
    print(result0);
    print(result1);
    print(result2);
}
