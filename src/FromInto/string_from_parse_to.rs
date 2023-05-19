use std::{
    fmt::{self, Error},
    num::{ParseFloatError, ParseIntError},
    str::FromStr,
};

#[derive(Debug)]
struct Circle {
    radius: i32,
}

impl fmt::Display for Circle {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Circle of radius {}", self.radius)
    }
}

#[derive(Debug)]
struct Point {
    x: f32,
    y: f32,
}

impl FromStr for Point {
    type Err = ParseFloatError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let coords: Vec<&str> = s
            .trim_matches(|p| p == '(' || p == ')')
            .split(',')
            .collect();

        let x_fromstr = coords[0].trim().parse::<f32>()?;
        let y_fromstr = coords[1].trim().parse::<f32>()?;

        Ok(Point {
            x: x_fromstr,
            y: y_fromstr,
        })
    }
}

fn main() {
    let circle = Circle { radius: 10 };

    let str = circle.to_string();
    // let str : String = circle.parse().unwrap();

    // let point = Point { x: 12, y: 22 };

    let p1 = Point::from_str("(1, 2)").expect("数字转化错误");
    let p2: Point = "(22.1,12.5)".parse().expect("转化失败");
    println!("{}, p1 :{:?}  p2:{:?}", str, p1, p2);
}
