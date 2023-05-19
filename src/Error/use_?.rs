use std::num::ParseIntError;

fn multiply(first: &str, second: &str) -> Result<i32, ParseIntError> {
    // first.parse::<i32>().and_then(|first_number| {
    //     second
    //         .parse::<i32>()
    //         .map(|second_number| first_number * second_number)
    // })
    let first = first.parse::<i32>()?;
    let second = second.parse::<i32>()?;

    Ok(first * second)
}

fn print_result(value: Result<i32, ParseIntError>) {
    match value {
        Ok(v) => println!("value {}", v),
        Err(e) => println!("{}", e),
    }
}

fn main() {
    let a = "22";
    let b = "5";
    let result = multiply(a, b);

    print_result(result);
}
