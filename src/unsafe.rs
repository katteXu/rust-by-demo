use std::slice;

/**
 * unsafe
 * 解引用裸指针
 * 通过FFi调用函数
 * 调用不安全的函数
 * 内联汇编
 */

fn main() {
    // 解引用裸指针
    // let raw_p: *const u32 = &10;

    // unsafe {
    //     println!("{:?},{}", raw_p, *raw_p);
    //     assert!(*raw_p == 10);
    // }

    // 调用unsafe函数
    let some_vector = vec![1, 2, 3, 4];

    let pointer = some_vector.as_ptr();
    let length = some_vector.len();
    unsafe {
        let my_slice = slice::from_raw_parts(pointer, length - 1);
        println!("调用函数{:?}", my_slice);
        println!("{:?}", some_vector.as_slice());
    }
}
