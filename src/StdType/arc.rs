use std::{sync::Arc, thread};

fn main() {
    let apple = Arc::new("the same apple");

    for i in 0..10 {
        let mut apple = Arc::clone(&apple);

        thread::spawn(move || {
            println!("{:?}", apple);
        });
    }
}
