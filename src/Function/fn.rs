fn main() {
    fizzbuzz_to(100);
}

fn fizzbuzz_to(n: i32) {
    for n in 0..=n {
        fizzbuzz(n);
    }
}

fn fizzbuzz(n: i32) {
    if divisible_by(n, 15) {
        println!("fizzbuzz");
    } else if divisible_by(n, 3) {
        println!("fizz");
    } else if divisible_by(n, 5) {
        println!("buzz");
    } else {
        println!("{}", n);
    }
}

fn divisible_by(target: i32, rhs: i32) -> bool {
    if target == 0 {
        return false;
    }
    target % rhs == 0
}
