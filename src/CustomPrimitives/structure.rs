/**
 * structure
 */

#[derive(Debug)]
struct Person {
    name: String,
    age: i32,
}

struct Unit;

struct Pair(i32, f32);

#[derive(Debug)]
struct Point {
    x: f32,
    y: f32,
}

impl Point {
    fn square(&self, width: f32) -> Rectangle {
        Rectangle {
            top_left: Point {
                x: self.x,
                y: self.y,
            },
            bottom_right: Point {
                x: &self.x + width,
                y: &self.y + width,
            },
        }
    }
}

struct Rectangle {
    top_left: Point,
    bottom_right: Point,
}

impl Rectangle {
    fn rect_area(&self) -> f32 {
        let width = (self.bottom_right.x - self.top_left.x).abs();
        let height = (self.bottom_right.y - self.top_left.y).abs();
        // width
        println!("width: {}, height: {}", width, height);
        width * height
    }
}

fn main() {
    let name = String::from("Peter");
    let age = 27;
    let peter = Person { name, age };
    println!("{:?}", peter);

    let point = Point { x: 1.0, y: 2.0 };

    println!("point {:0.1} , {:0.1}", point.x, point.y);

    let bottom_right = Point { x: 2.2, ..point };

    println!("bottom_right {:.1}, {:.1}", bottom_right.x, bottom_right.y);

    let Point {
        x: left_edge,
        y: top_edge,
    } = point;

    let _rectangle = Rectangle {
        top_left: Point { x: 3.5, y: 1.1 },
        bottom_right,
    };

    let area = _rectangle.rect_area();
    println!("area: {}", area);

    let _rect = point.square(2.5);
    println!("rect: {:?},{:?}", _rect.top_left, _rect.bottom_right);
    println!("_react area {}", _rect.rect_area());
    let _unit = Unit;

    let pair = Pair(1, 1.1);
}
