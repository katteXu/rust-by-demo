/**
 * 在Rust中所有的内存都是栈分配
 * 可以通过Box<T> 来进行堆分配  装箱
 * Box 是一个智能指针
 * 被装箱的值 可以用*运算符进行解引用；这会移除掉一层装箱
 */

#[derive(Debug)]
struct Point {
    x: f64,
    y: f64,
}

struct Rectangle {
    p1: Point,
    p2: Point,
}

fn origin() -> Point {
    Point { x: 0.0, y: 0.0 }
}

fn boxed_origin() -> Box<Point> {
    Box::new(Point { x: 0.0, y: 0.0 })
}

fn main() {
    let point = origin();

    let rectangle = Rectangle {
        p1: origin(),
        p2: Point { x: 3.9, y: 4.0 },
    };

    let boxed_rectangle = Box::new(Rectangle {
        p1: origin(),
        p2: origin(),
    });

    let boxed_point = Box::new(origin());

    let mut boxed_in_a_box = Box::new(boxed_origin());
    // boxed_in_a_box.as_mut().x = 1.2;
    // println!("boxed in a box {:#?}", boxed_in_a_box);

    
}
