/**
 * block
 * expr 用于表达式
 * ident 用于变量名或函数名
 * item
 * pat (模式 pattern)
 * path
 * stmt (语句 statement)
 * tt (标记树 token tree)
 * ty (类型 type)
 */

macro_rules! create_function {
    ($func_name:ident) => {
        fn $func_name() {
            println!("{:?}", stringify!($func_name));
        }
    };
}

macro_rules! print_result {
    ($expression:expr) => {
        println!("{:?} = {:?}", stringify!($expression), $expression);
    };
}

#[derive(Debug)]
struct A {
    name: String,
    age: u8,
}
fn main() {
    let hahaa = A {
        name: "name".into(),
        age: 22,
    };
    create_function!(abc);
    print_result!(hahaa);
    print_result!({
        let x = 32;
        x * x + 2 * x - 1
    });
    abc();
}
