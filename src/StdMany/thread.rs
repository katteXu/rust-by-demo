use std::thread;

fn main() {
    let mut children = vec![];

    for i in 0..10 {
        children.push(thread::spawn(move || {
            println!("this is thread number {}", i);
        }));
    }

    for child in children {
        let _ = child.join();
    }
}
