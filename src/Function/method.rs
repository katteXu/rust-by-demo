#[derive(Debug)]
struct Point {
    x: f64,
    y: f64,
}

impl Point {
    fn new(x: f64, y: f64) -> Point {
        Point { x, y }
    }

    fn origin() -> Point {
        Point::new(0.0, 0.0)
    }
}

// 形状
#[derive(Debug)]
struct Rectangle {
    p1: Point,
    p2: Point,
}

impl Rectangle {
    // 求面积
    fn area(&self) -> f64 {
        let Point { x: x1, y: y1 } = self.p1;
        let Point { x: x2, y: y2 } = self.p2;

        ((x1 - x2) * (y1 - y2)).abs()
    }

    // 周长
    fn perimeter(&self) -> f64 {
        let Point { x: x1, y: y1 } = self.p1;
        let Point { x: x2, y: y2 } = self.p2;

        2.0 * ((x1 - x2).abs() + (y1 - y2).abs())
    }

    fn translate(&mut self, x: f64, y: f64) {
        self.p1.x += x;
        self.p1.y += y;
        self.p2.x += x;
        self.p2.y += y;
    }
}

#[derive(Debug)]
struct Pair(Box<i32>, Box<i32>);

impl Pair {
    fn destory(self) {
        let Pair(a, b) = self;

        println!("destory: a = {}, b = {}", a, b);
    }
}

fn main() {
    let rectangle = Rectangle {
        p1: Point::origin(),
        p2: Point::new(3.0, 4.0),
    };

    println!("Rectangle: {:#?}", rectangle);
    println!("Rectangle.area: {}", rectangle.area());
    println!("Rectangle.perimeter: {}", rectangle.perimeter());

    let mut square = Rectangle {
        p1: Point::origin(),
        p2: Point::new(1.0, 1.0),
    };

    println!("Square: {:#?}", square);

    // 正常运行！可变对象可以调用可变方法
    square.translate(1.0, 1.0);

    println!("Square: {:#?}", square);

    let pair = Pair(Box::new(1), Box::new(2));

    pair.destory();

    // println!("pair: {:#?}", pair);

}
