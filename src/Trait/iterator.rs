use std::ops::Range;

#[derive(Debug)]
struct Fibonacci {
    curr: u128,
    next: u128,
}

impl Iterator for Fibonacci {
    type Item = u128;
    fn next(&mut self) -> Option<Self::Item> {
        let new_next = self.curr + self.next;
        self.curr = self.next;
        self.next = new_next;
        Some(self.curr)
    }
}

fn main() {
    let mut sequence: Range<i32> = 0..3;

    // while let Some(n) = sequence.next() {
    //     println!("{}", n);
    // }

    let mut f = Fibonacci { curr: 1, next: 1 };

    f.skip(1).take(5).for_each(|f| {
        println!("{}", f);
    });
    // println!("f next {:?}", f.take(4));
    // println!("f next {:?}", f.next());
    // println!("f next {:?}", f.next());
    // println!("f next {:?}", f.next());

    // while let Some(n) = f.next() {
    //     println!("f next {:?}", n);
    // }
}
