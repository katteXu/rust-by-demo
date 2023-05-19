use std::{
    error::{self, Error},
    fmt::Display,
};

type Result<T> = std::result::Result<T, Box<dyn error::Error>>;

#[derive(Debug, Clone)]
struct EmptyVec;

impl Display for EmptyVec {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "empty vector")
    }
}

impl Error for EmptyVec {
    fn description(&self) -> &str {
        "empty vector"
    }

    fn cause(&self) -> Option<&dyn Error> {
        None
    }
}

fn double_first(v: Vec<&str>) -> Result<i32> {
    let result = v
        .first()
        .ok_or_else(|| EmptyVec.into())
        .and_then(|s| s.parse::<i32>().map_err(|e| e.into()).map(|v| 2 * v));
    result
}

fn print(result: Result<i32>) {
    match result {
        Ok(n) => println!("The first doubled is {}", n),
        Err(e) => println!("Error: {}", e),
    }
}

fn main() {
    let numbers = vec!["42", "93", "18"];
    let empty = vec![];
    let strings = vec!["tofu", "93", "18"];

    print(double_first(numbers));
    print(double_first(empty));
    print(double_first(strings));
}
