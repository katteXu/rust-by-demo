/**
 * 在进行赋值（let x = y）或通过值来传递函数参数（foo(x)）的时候，资源的所有权（ownership）会发生转移。
 * 按照 Rust 的说法，这被称为资源的移动（move）。
 */

fn destory_box(c: Box<i32>) {
    println!("Destroying box {}", c);
}
fn main() {
    let my_box = Box::new(123);

    let x = 3i32;

    // 将 `x` *复制*到 `y`——不存在资源移动
    let y = x;

    // `a` 是一个指向堆分配的整数的指针
    let a = Box::new(5i32);

    let b = a;
    // 把 `a` 的指针地址（而非数据）复制到 `b`。现在两者都指向
    // 同一个堆分配的数据，但是现在是 `b` 拥有它。

    // 报错！`a` 不能访问数据，因为它不再拥有那部分堆上的内存。
    //println!("a contains: {}", a);
    // 试一试 ^ 去掉此行注释

    destory_box(b);

    // 此时堆内存已经被释放，这个操作会导致解引用已释放的内存，而这是编译器禁止的。
    // 报错！和前面出错的原因一样。
    //println!("b contains: {}", b);
    // 试一试 ^ 去掉此行注释
}
