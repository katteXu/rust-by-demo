use std::fmt::Debug;
use std::ops::Add;
// DRY

macro_rules! op {
    ($func:ident,$bound:ident,$method:ident) => {
        fn $func<T: $bound<T, Output = T> + Copy>(xs: &mut Vec<T>, ys: &Vec<T>)
        where
            T: Debug,
        {
            // ass
            for (x, y) in xs.iter_mut().zip(ys.iter()) {
                println!("x: $x {:?}", x);
                println!("y: $y {:?}", y);
                *x = $bound::$method(*x, *y);
                // *x = x.$method(*y);
            }
        }
    };
}

op!(add_assign, Add, add);

fn main() {
    let mut x = vec![1, 2, 3];
    let mut y = vec![1, 2, 3];
    add_assign(&mut x, &y);

    println!("x: {:?}, y :{:?}", x, y);
}
