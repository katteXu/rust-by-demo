use std::{num::ParseIntError, vec};

type Result<T> = std::result::Result<T, DoubleError>;
#[derive(Debug)]
enum DoubleError {
    EmptyVec,
    Parse(ParseIntError),
}

impl std::fmt::Display for DoubleError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DoubleError::EmptyVec => write!(f, "please use a vector with at least one element"),
            DoubleError::Parse(ref e) => e.fmt(f),
        }
    }
}

impl std::error::Error for DoubleError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            DoubleError::EmptyVec => None,
            DoubleError::Parse(ref e) => Some(e),
        }
    }
}

impl From<ParseIntError> for DoubleError {
    fn from(err: ParseIntError) -> Self {
        DoubleError::Parse(err)
    }
}

fn double_first(vec: Vec<&str>) -> Result<i32> {
    let first = vec.first().ok_or(DoubleError::EmptyVec)?;
    let parsed = first.parse::<i32>()?;

    Ok(2 * parsed)
}

fn print(result: Result<i32>) {
    match result {
        Ok(n) => println!("{}", n),
        Err(e) => println!("{}", e),
    }
}

fn main() {
    let numbers = vec!["42", "44"];
    let empty: Vec<&str> = vec![];
    let strings = vec!["a", "b", "c"];

    print(double_first(numbers));
    print(double_first(strings));
    print(double_first(empty));
}
