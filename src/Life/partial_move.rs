fn main() {
    #[derive(Debug)]
    struct Person {
        name: String,
        age: u8,
    }

    let person = Person {
        name: String::from("bobo"),
        age: 20,
    };

    let Person { name, age: ref age } = person;

    println!("{:?}", person.age);
}
