/**
 * 多数情况下，我们更希望能访问数据，同时不取得其所有权。为实现这点，Rust 使用了借用（borrowing）机制。
 * 对象可以通过引用（&T）来传递，从而取代通过值（T）来传递。
 */

// 此函数取得一个 box 的所有权并销毁它
fn eat_box_i32(boxed_i32: Box<i32>) {
    println!("box {}", boxed_i32);
}

// 此函数借用了一个 i32 类型
fn borrow_i32(borrowed_i32: &i32) {
    println!("borrowed value {}", borrowed_i32);
}

fn main() {
    let boxed_i32 = Box::new(5i32);
    let stacked_i32 = 6i32;

    borrow_i32(&boxed_i32);
    borrow_i32(&stacked_i32);

    {
        let _ref_to_i32: &i32 = &boxed_i32;

        // eat_box_i32(boxed_i32);

        borrow_i32(_ref_to_i32);
    }

    eat_box_i32(boxed_i32);
}
