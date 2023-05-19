use std::mem;

fn analyze_slice(slice: &[i32]) {
    println!("{}", slice[0]);
    println!("{}", slice.len());
}

fn main() {
    let xs = [0, 1, 2, 3, 4];
    let ys = [0; 500];

    println!("{}", xs[0]);
    println!("{}", xs[1]);

    println!("{}", xs.len());

    // 判断内存中的大小
    println!("{} bytes", mem::size_of_val(&xs));

    analyze_slice(&xs);

    analyze_slice(&ys[1..4]);

    let result = xs.get(5).unwrap_or(&-1);
    println!("{}", result);
}
