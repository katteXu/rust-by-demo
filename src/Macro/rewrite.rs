macro_rules! test {
    ($left:expr;  加 $right:expr) => {
        println!("{:?} 加 {:?} 是 {:?}", $left, $right, $left + $right);
    };
    ($left:expr; - $right:expr) => {
        println!("{:?} 减 {:?} 是 {:?}", $left, $right, $left - $right);
    };
}

fn main() {
    test!(1;    加 2);
    test!(1; - 2);
}
