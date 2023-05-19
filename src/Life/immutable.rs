fn main() {
    let mut my_box = Box::new(5);

    *my_box = 6;

    println!("{}", *my_box);
}
