use std::{
    sync::mpsc::{self, Receiver, Sender},
    thread,
};

fn main() {
    let (tx, rx): (Sender<i32>, Receiver<i32>) = mpsc::channel();

    for id in 0..3 {
        let thread_tx = tx.clone();

        thread::spawn(move || {
            thread_tx.send(id).unwrap();

            println!("thread {} finished", id);
        });
    }

    let mut ids = Vec::with_capacity(3);

    for _ in 0..3 {
        ids.push(rx.recv().unwrap());
    }

    println!("{:?}", ids);
}
