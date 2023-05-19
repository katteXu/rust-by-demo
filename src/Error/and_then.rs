fn main() {
    let vec = vec![1, 2, 3];

    let result = vec.get(0).and_then(|&x| Some(x + 1));

    println!("{:?}", result);
}
