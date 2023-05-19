struct UnPrintable(i32);

#[derive(Debug)]
struct DebugPrintable(i32);

#[derive(Debug)]
struct Structure(i32);

#[derive(Debug)]
struct Deep(Structure);

#[derive(Debug)]
struct Person<'a> {
    name: &'a str,
    age: u8,
}

impl<'a> Person<'a> {
    fn new(name: &'a str, age: u8) -> Self {
        Self { name, age }
    }
}
fn main() {
    println!("{:?} months is a year", 12);

    println!(
        "{1:?} {0:?} is the {actor:?} name.",
        "Slater",
        "Christian",
        actor = "actor's"
    );

    println!("Now {:?} will print!", Structure(3));

    println!("Now {:?} will print!", Deep(Structure(7)));

    let name = "Peter";
    let age = 32;
    let peter = Person::new(name, age);

    println!("{:#?}", peter);
}
