struct Point {
    x: i32,
    y: i32,
    common: String,
}

fn main() {
    let c = 'Q';

    // 等同效果
    let ref ref_c = c;
    let ref_c2 = &c;

    let point = Point {
        x: 1,
        y: 2,
        common: "h".into(),
    };

    let Point { x, y, common } = &point;
    let Point { x, ref y, ref common } = point;

    println!(
        "point.x: {} y: {} ,common:{}",
        point.x, point.y, point.common
    );
}
