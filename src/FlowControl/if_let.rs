/**
 * if let
 */

enum Foo {
    Bar,
    Baz,
    Qux(u32),
}

enum Foo2 {
    Bar,
}
fn main() {
    let optional = Some(7);

    match optional {
        Some(i) => println!("i = {}", i),
        None => println!("None"),
    }

    let number = Some(7);
    let letter: Option<i32> = None;
    let emoticon: Option<i32> = None;

    if let Some(i) = number {
        println!("number matched  {}", i);
    }

    if let Some(i) = letter {
        println!("Matched {}", i);
    } else {
        println!("Didn't match a number. Let's go with a letter!");
    }

    let i_like_letters = false;

    if let Some(i) = emoticon {
        println!("emoticon matched {}", i);
    } else if i_like_letters {
        println!("emoticon matched a letter");
    } else {
        println!("emoticon matched nothing");
    }

    let a = Foo::Baz;
    let b = Foo::Bar;
    let c = Foo::Qux(12);
    let d = Foo2::Bar;
    if let Foo::Bar = a {
        println!("a = Bar");
    }

    if let Foo::Bar = b {
        println!("b = Bar");
    }

    if let Foo::Qux(i) = c {
        println!("c is {}", i);
    }

    if let Foo2::Bar = d {
        println!("a is foobar");
    }
}
