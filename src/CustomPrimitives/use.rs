#[derive(Debug)]
enum Status {
    Rich,
    Poor,
}

enum Work {
    Clivilian,
    Soldier,
}

fn main() {
    use Status::{Poor, Rich};

    use Work::*;

    let status = Poor;
    let work = Clivilian;

    match status {
        Rich => todo!(),
        Poor => todo!(),
    }

    match work {
        Clivilian => todo!(),
        Soldier => todo!(),
    }

    // println!("{:?}", poor);
}
