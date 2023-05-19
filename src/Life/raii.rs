fn create_box() {
    let _box1 = Box::new(3i32);
}

struct ToDrop;

impl Drop for ToDrop {
    fn drop(&mut self) {
        println!("释放完毕")
    }
}

fn main() {
    let x = ToDrop;

    println!("Mad a ToDrop");
}
