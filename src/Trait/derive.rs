/**
 * 特征
 * 比较 trait: Eq, PartialEq, Ord, PartialOrd
 * Clone, 用来从 &T 创建副本 T。
 * Copy，使类型具有 “复制语义”（copy semantics）而非 “移动语义”（move semantics）。
 * Hash，从 &T 计算哈希值（hash）。
 * Default, 创建数据类型的一个空实例。
 * Debug，使用 {:?} formatter 来格式化一个值。
 */

#[derive(PartialEq, PartialOrd)]
struct Centimeters(f64, i32);

#[derive(PartialEq)]
struct Person {
    name: String,
    age: i32,
}
fn main() {
    let a = Centimeters(11.0, 12);
    let b = Centimeters(12.0, 13);

    let p1 = Person {
        name: String::from("123"),
        age: 12,
    };
    let p2 = Person {
        name: String::from("1232"),
        age: 12,
    };

    // if p1 > p2 {
    //     println!("同一个人");
    // } else {
    //     println!("不同人");
    // }

    println!("{}", a >= b);
}
