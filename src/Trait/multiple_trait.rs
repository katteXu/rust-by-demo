trait Trait1 {
    fn get(&self) -> i32;
}

trait Trait2 {
    fn get(&self) -> String;
}

trait Trait3 {
    fn get(&self) -> bool;
}

struct Person;

impl Trait1 for Person {
    fn get(&self) -> i32 {
        32
    }
}

impl Trait2 for Person {
    fn get(&self) -> String {
        "String".into()
    }
}

impl Trait3 for Person {
    fn get(&self) -> bool {
        true
    }
}

fn main() {
    let p1 = Person;
    let get1 = <Person as Trait1>::get(&p1);
    let get2 = <Person as Trait2>::get(&p1);
    let get3 = <Person as Trait3>::get(&p1);

    // p1.get();
    println!("{},{},{}", get1, get2, get3);
}
