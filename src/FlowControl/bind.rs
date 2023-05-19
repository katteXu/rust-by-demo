fn age() -> u32 {
    15
}

fn some_number() -> Option<u32> {
    Some(32)
}

fn main() {
    match age() {
        0 => println!("zero"),
        n @ 1..=12 => println!("1-12 :{} ", n),
        n @ 13..=19 => println!("13-19 :{} ", n),
        n => println!("other"),
    }

    match some_number() {
        Some(n @ 32) => println!("some number: {}", n),
        Some(n) => println!("some other number: {}", n),
        None => println!("none"),
    };
}
