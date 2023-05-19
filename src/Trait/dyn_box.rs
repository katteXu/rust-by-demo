trait Animal {}

struct A;

impl Animal for A {}

fn get_animal() -> Box<dyn Animal> {
    let a = A;
    Box::new(a)
}
fn main() {}
