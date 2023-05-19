/**
 * 卫语句 guard
 */

fn main() {
    let pair = (2, 12);

    println!("Tell me about {:?}", pair);

    match pair {
        (x, y) if x == y => println!("These are twins"),
        (x, y) if x + y == 0 => println!("Antimatter, kaboom!"),
        (x, _) if x % 2 == 1 => println!("The first one is odd"),
        _ => println!("No correlation..."),
    }
}
