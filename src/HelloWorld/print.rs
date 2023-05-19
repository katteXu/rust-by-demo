#[derive(Debug)]
struct Structure(i32);

fn main() {
    println!("{}, days", 31);

    print!("{0}, this is {1}, {1}, this is  {0}", "Alice", "Bob");

    println!(
        "{subject},{verb},{object}",
        object = "the lazy dog",
        subject = "the quick brown fox",
        verb = "jumps over"
    );

    println!("{} of {:b} people know binary, the other half don't", 1, 2);

    println!("My name is {0} ,{1} {0}", "Bond", "James");

    println!(" This struct `{:?}` won't print ...", Structure(3));

    let pi = 3.1415926;
    println!("Pi is roughly {:0.3}", pi);
    let first = 0.3;
    println!("Hello {:.1$}", 123.22222, 3);

    let str = format!("Hello {1} is {2:.0$}", 5, "x", 0.01);
    println!("{str}");

    
}
