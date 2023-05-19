fn main() {
    let _immuteable_binding = 1;

    let mut mutable_binding = 1;

    println!("{}", mutable_binding);

    mutable_binding += 1;

    println!("{}", mutable_binding);
}
