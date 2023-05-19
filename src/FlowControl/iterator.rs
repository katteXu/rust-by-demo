use std::fmt::format;

/**
 * iter
 * into_iter
 * iter_mut
 */

fn main() {
    let mut names = ["one", "two", "three"];

    for name in names.iter() {
        match name {
            &"one" => {
                println!("one");
            }
            _ => println!("Hello {}", name),
        }
    }

    for name in names.into_iter() {
        match name {
            "one" => println!("There is a rustacean among us!"),
            _ => println!("Hello {}", name),
        }
    }

    for name in names.iter_mut() {
        let mut res = String::from("Hello");
        *name = match name {
            &mut "one" => "There is a rustacean among us!",
            _ => {
                "hello"
            }
        }
    }
    println!("names: {:?}", names);

    println!("{names:?}");
}
