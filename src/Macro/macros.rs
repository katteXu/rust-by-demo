/**
 * 为什么宏是有用的？

 * 1. 不写重复代码（DRY，Don't repeat yourself.）。
 * 很多时候你需要在一些地方针对不同 的类型实现类似的功能，这时常常可以使用宏来避免重复代码（稍后详述）。
 * 2. 领域专用语言（DSL，domain-specific language）。
 * 宏允许你为特定的目的创造特定的 语法（稍后详述）。
 * 3. 可变接口（variadic interface）。
 * 有时你需要能够接受不定数目参数的接口，比如 println!，根据格式化字符串的不同，它需要接受任意多的参数（稍后详述）。
 */

macro_rules! say_hello {
    // `()` 表示此宏不接受任何参数。
    () => {
        // 此宏将会展开成这个代码块里面的内容。
        println!("Hello!");
    };
}

fn main() {
    // 这个调用将会展开成 `println("Hello");`!
    say_hello!();

    println!("World!");
}
