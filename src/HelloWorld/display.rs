use std::fmt;

struct Structure(i32);

impl fmt::Display for Structure {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Structure: {}", self.0)
    }
}

#[derive(Debug)]
struct MinMax(i64, i64);

impl fmt::Display for MinMax {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.0, self.1)
    }
}

#[derive(Debug)]
struct Point2D {
    x: f64,
    y: f64,
}

impl fmt::Display for Point2D {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "x: {}, y: {}", self.x, self.y)
    }
}

impl fmt::Binary for Point2D {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({:b}, {:b})", self.x as i32, self.y as i32)
    }
}

#[derive(Debug)]
struct Complex {
    real: f64,
    imag: f64,
}

impl fmt::Display for Complex {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} + {}i", self.real, self.imag)
    }
}

fn main() {
    let i = Structure(5);
    let vecs = vec![1, 2, 3, 4];
    println!("{i} ,{num:?}", num = vecs[0]);

    let minmax = MinMax(12, 22);
    let point2d = Point2D { x: 12.0, y: 22.0 };

    println!("Compare structures:");
    println!("Display:{}", minmax);
    println!("Debug:{:?}", minmax);

    let big_range = MinMax(-300, 300);
    let small_range = MinMax(-3, 3);
    // println!("minmax:{}, point2d:{}", minmax, point2d);

    // println!("big range is {big_range}, small range is {small_range}");

    let point = Point2D { x: 3.3, y: 7.2 };

    println!("Compare points:");
    println!("Display {}", point);
    println!("Debug {:?}", point);

    println!("What does Point2D look like in binary: {:b}?", point);

    let complex = Complex {
        real: 3.3,
        imag: 7.2,
    };
    println!("Display: {}", complex);
    println!("Debug :{:?}", complex);
}
