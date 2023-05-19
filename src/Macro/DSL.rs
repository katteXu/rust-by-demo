macro_rules! calculate {
    (eval $e:expr) => {
        {
            let val:usize = $e;
            println!("{} = {}", stringify!{$e}, val);
        }
    };
}

fn main() {
    calculate! {
        eval 1 + 2
    }
    let x = 1;
    let y = 2;
    calculate!{
        eval x + y
    }
}
