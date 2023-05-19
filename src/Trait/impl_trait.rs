trait MyTrait {
    fn value(self) -> String;
}

struct A(i32);
impl MyTrait for A {
    fn value(self) -> String {
        self.0.to_string()
    }
}
struct B {
    age: u32,
}
impl MyTrait for B {
    fn value(self) -> String {
        self.age.to_string()
    }
}
fn get_someting<T: MyTrait>(x: T) -> impl MyTrait {
    x
}

fn combine_vecs(a: Vec<i32>, b: Vec<i32>) -> impl Iterator<Item = i32> {
    a.into_iter().chain(b.into_iter()).cycle()
}

fn main() {
    let a = A(32);
    let b = B { age: 12 };
    let some = get_someting(a);
    let some2 = get_someting(b);
    println!("a: {}, b: {}", some.value(), some2.value());

    let v1 = vec![1, 2, 3, 4];
    let v2 = vec![5, 6, 7, 8];

    let mut result = combine_vecs(v1, v2);

    // 一个环
    while let Some(n) = result.next() {
        println!("{:?}", n);
    }
}

fn make_closure(y: i32) -> impl Fn(i32) -> i32 {
    let closure = move |x| x + y;
    closure
}

fn double_positives<'a>(numbers: &'a Vec<i32>) -> impl Iterator<Item = i32> + 'a {
    numbers.iter().filter(|x| x > &&0).map(|x| x * 2)
}
