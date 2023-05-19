use List::*;

enum List {
    Cons(u32, Box<List>),
    Nil,
}

impl List {
    fn new() -> List {
        Nil
    }

    fn prepend(self, elem: u32) -> List {
        Cons(elem, Box::new(self))
    }

    fn append(self, elem: u32) -> List {
        match self {
            Cons(head, tail) => Cons(head, Box::new(tail.append(elem))),
            Nil => Cons(elem, Box::new(Nil)),
        }
    }
    fn len(&self) -> u32 {
        match *self {
            Cons(_, ref tail) => 1 + tail.len(),
            Nil => 0,
        }
    }

    fn stringify(&self) -> String {
        match *self {
            Cons(head, ref tail) => {
                format!("{} -> {}", head, tail.stringify())
            }
            Nil => {
                format!("Nil")
            }
        }
    }
}

fn main() {
    let mut list = List::new();
    list = list.prepend(1);
    list = list.append(3);
    list = list.prepend(2);
    list = list.append(4);
    list = list.prepend(5);
    list = list.append(6);
    let str = list.stringify();

    println!("len: {}", list.len());
    println!("{}", str);
}
