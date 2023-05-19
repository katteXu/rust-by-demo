fn check_division(dividend: i32, divisor: i32) -> Option<i32> {
    if divisor == 0 {
        None
    } else {
        Some(dividend / divisor)
    }
}

fn try_division(dividend: i32, divisor: i32) {
    match check_division(dividend, divisor) {
        None => println!("{} / {} failed", dividend, divisor),
        Some(quotient) => println!("{} / {} = {}", dividend, divisor, quotient),
    }
}

fn main() {
    try_division(4, 2);
    try_division(1, 0);

    let none: Option<i32> = None;
    let _equialent_none = None::<i32>;
    let optional_float = Some(0f32);

    println!(
        "{:?}  unwraps {:?}",
        optional_float,
        optional_float.unwrap()
    );

    // 解包None 会引发 panic!
    println!("{:?} unwraps {:?}", none, none.unwrap());

    println!("{}", "abc");
}
