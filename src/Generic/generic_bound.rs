use std::fmt::Debug;
use std::fmt::Display;

fn printer<T: Display>(t: T) {
    println!("{}", t);
}

#[derive(Debug)]
struct Rectangle {
    length: f64,
    height: f64,
}

trait HasArea {
    fn area(&self) -> f64;
}

fn print_debug<T: Debug>(t: &T) {
    println!("{:?}", t);
}

impl HasArea for Rectangle {
    fn area(&self) -> f64 {
        let result = &self.length * &self.height;
        result
    }
}

fn area<T: HasArea>(t: &T) -> f64 {
    t.area()
}

fn compare_prints<T: Debug + Display>(t: &T) {
    println!("Debug: `{:?}`", t);
    println!("Display: `{}`", t);
}

fn compare_types<T: Debug, U: Debug>(t: &T, u: &U) {
    println!("t: `{:?}`", t);
    println!("u: `{:?}`", u);
}

fn main() {
    let rectangle = Rectangle {
        length: 2.5,
        height: 4.0,
    };

    print_debug(&rectangle);
    printer(area(&rectangle));
}
