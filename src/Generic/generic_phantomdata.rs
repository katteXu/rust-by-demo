use std::{marker::PhantomData, ops::Add};

/// 创建空枚举类型来表示单位。
#[derive(Debug, Clone, Copy)]
struct Inch;
#[derive(Debug, Clone, Copy)]
struct Mm;

#[derive(Debug, Clone, Copy)]
struct Length<Unit>(f64, PhantomData<Unit>);

impl<Unit> Add for Length<Unit> {
    type Output = Length<Unit>;
    fn add(self, rhs: Self) -> Self::Output {
        Length(self.0 + rhs.0, PhantomData)
    }
}

fn main() {
    let one_foot: Length<Inch> = Length(12.0, PhantomData);

    let one_meter: Length<Mm> = Length(1000.0, PhantomData);

    let two_foot = one_foot + one_foot;
    let two_meter = one_meter + one_meter;

    println!("{:?} , {:?}", two_foot, two_meter);
    // let err = one_foot + one_meter;
}
