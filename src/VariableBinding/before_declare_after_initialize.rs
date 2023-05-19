fn main() {
    let a_binding;

    {
        let x = 2;

        // copy
        a_binding = x;
    }

    println!("a binding {}", a_binding);

    let another_binding;

    // 没有变量提升 报错
    // println!(" another binding {}", another_binding);

    another_binding = 1;

    println!("another binding {}", another_binding);
}
