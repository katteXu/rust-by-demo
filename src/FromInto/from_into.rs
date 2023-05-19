/**
 * From 根据另一种类型生成自己
 * Into 基本一样的。。。
 */

#[derive(Debug)]
struct Number {
    value: i32,
}

impl From<i32> for Number {
    fn from(value: i32) -> Self {
        Number { value }
    }
}

fn main() {
    let my_str = "hello";
    let my_string = String::from(my_str);
    let my_String: String = my_str.into();
    // let my_String2 = "hello" as String;

    // 从 10 (A) 得到 Number (B)
    let number = Number::from(10);
    // 将 10 (A) 转化成 Number (B)
    let number2: Number = 10.into();
    println!("{number:?}");

    // let a = 5.into::<Number>();
    // let my_str2: &str = my_string.into();
}
