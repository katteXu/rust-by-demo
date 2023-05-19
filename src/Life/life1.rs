fn printer<'a, 'b>(x: &'a i32, y: &'b i32) {
    println!("{},{}", x, y);
}

fn failed_borrow<'a>() {
    let _x = 12;

    // 报错：`_x` 的生命周期不够长
    // let y: &'a i32 = &_x;
    // 在函数内部使用生命周期 `'a` 作为显式类型标注将导致失败，因为 `&_x` 的
    // 生命周期比 `y` 的短。短生命周期不能强制转换成长生命周期。
}

fn add_one<'a>(x: &'a mut i32) {
    *x += 1;
}

fn pass_x<'a>(x: &'a i32, y: &i32) -> &'a i32 {
    x
}

struct Owner(i32);

impl Owner {
    fn add_one(&mut self) {
        self.0 += 1;
    }

    fn print(&self) {
        println!("{}", self.0);
    }
}

struct NamedBorrow<'a> {
    x: &'a i32,
    y: &'a i32,
}

enum Nither<'a> {
    Num(i32),
    Ref(&'a i32),
}

#[derive(Debug)]
struct Borrowed<'a> {
    x: &'a i32,
}

impl<'a> Default for Borrowed<'a> {
    fn default() -> Self {
        Self { x: &10 }
    }
}

#[derive(Debug)]
struct Ref<'a, T: 'a>(&'a T);

fn print<T>(t: T)
where
    T: std::fmt::Debug,
{
    println!("{:?}", t);
}

fn print_ref<'a, T>(t: &'a T)
where
    T: std::fmt::Debug,
{
    println!("{:?}", t);
}
fn main() {
    let (four, nine) = (4, 9);

    printer(&four, &nine);

    failed_borrow();

    let mut owner = Owner(4);

    owner.add_one();

    owner.print();

    let borrow: Borrowed = Default::default();

    println!("{:#?}", borrow.x);

    let x = 7;
    let ref_x = Ref(&x);

    print_ref(&ref_x);
    print(ref_x);
}

// TODO ???
fn choose_first<'a, 'b>(first: &'a i32, _: &'b i32) -> &'a i32 {
    first
}

// 省略的生命周期
fn elided_input(x: &i32) {
    println!("`elided_input`: {}", x)
}

fn annotated_input<'a>(x: &'a i32) {
    println!("`annotated_input`: {}", x)
}

fn elided_pass(x: &i32) -> &i32 {
    x
}

fn annotated_pass<'a>(x: &'a i32) -> &'a i32 {
    x
}
