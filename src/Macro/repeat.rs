macro_rules! find_min {
    ($x:expr) => ($x);
    // `$x` 后面跟着至少一个 `$y,`
    // $(...),+ 包围起来，就可以匹配一个或多个用逗号隔开的表达式
    ($x:expr,$($y:expr),+) => (
        std::cmp::min($x,find_min!($($y),+))
    )
}
fn main() {
    println!("{}", find_min!('a', 'b', 'c', 'x'));
    println!("{}", find_min!(1));
    println!("{}", find_min!(2, 100, 22, 75, 21));
}
