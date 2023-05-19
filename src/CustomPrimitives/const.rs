/**
 * 常量
 * const   不可变
 * static  具有'static 生命周期 可以是可变的变量
 */

static LANGUAGE: &'static str = "RUST";
const THRESHOLD: i32 = 10;

fn is_big(n: i32) -> bool {
    n > THRESHOLD
}

fn main() {
    let n = 16;

    println!("staitc lanaguage : {}", LANGUAGE);
    println!("{} ", THRESHOLD);
    // unsafe {
    //     LANGUAGE = "Java";
    //     println!("{}", LANGUAGE);
    // }
}
