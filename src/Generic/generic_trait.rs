use std::fmt::{Debug, Display};

struct Empty;

#[derive(Debug)]
struct Null;

trait DoubleDrop<T> {
    fn double_drop(self, _: T);
}

impl<T, U> DoubleDrop<T> for U
where
    T: Debug,
{
    fn double_drop(self, x: T) {
        println!("double_drop,{:?}", x);
    }
}

fn main() {
    let empty = Empty;
    let null = Null;

    // 一次释放两个内存
    empty.double_drop(null);

    // empty;
    // null;
}
