fn main() {
    struct Foo {
        x: (u32, i32),
        y: u32,
    };

    let foo = Foo { x: (1, 2), y: 3 };
    let Foo { x: (a, b), y } = foo;

    let Foo { y: i, x: j } = foo;

    let Foo { y, .. } = foo;

    let Foo { x, .. } = foo;
}
