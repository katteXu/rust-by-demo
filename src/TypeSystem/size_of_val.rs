use std::mem;

fn main() {
    let x = 1u8;
    let y = 2u32;
    let z = 3.0f32;

    let i = 1;
    let f = 1.0;

    println!("size of `x` in bytes: {}", mem::size_of_val(&x));
    println!("size of `y` in bytes: {}", mem::size_of_val(&y));
    println!("size of `z` in bytes: {}", mem::size_of_val(&z));
    println!("size of `i` in bytes: {}", mem::size_of_val(&i));
    println!("size of `f` in bytes: {}", mem::size_of_val(&f));
    
}
