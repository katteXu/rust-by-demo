use std::ops::Add;

struct Foo;
struct Bar;

#[derive(Debug)]
struct FooBar;

#[derive(Debug)]
struct BarFoo;

impl Add<Foo> for Bar {
    type Output = BarFoo;
    fn add(self, rhs: Foo) -> Self::Output {
        println!("Bar + Foo = BarFoo");
        BarFoo
    }
}

impl Add<Bar> for Foo {
    type Output = FooBar;
    fn add(self, rhs: Bar) -> Self::Output {
        println!("Foo + Bar = FooBar");
        FooBar
    }
}

fn main() {
    let a = Bar + Foo;

    let b = Foo + Bar;

    println!("a {:?}", a);
    println!("b {:?}", b);
}
