fn main() {
    let strings = vec!["23", "def", "14"];

    let numbers: Result<Vec<_>, _> = strings
        .clone()
        .into_iter()
        .map(|s| s.parse::<i32>())
        .collect();
    let numbers2: Vec<_> = strings
        .into_iter()
        .filter_map(|s| s.parse::<i32>().ok())
        .collect();
    // println!("{:#?}", numbers);
    // println!("{:#?}", numbers2);

    let partition_strings = vec!["23", "def", "44"];

    let (numbers, errors): (Vec<_>, Vec<_>) = partition_strings
        .into_iter()
        .map(|s| s.parse::<i32>())
        .partition(Result::is_ok);
    let numbers: Vec<_> = numbers.into_iter().map(Result::unwrap).collect();
    let errors: Vec<_> = errors.into_iter().map(Result::unwrap_err).collect();
    println!("{:#?}", numbers);
    println!("{:#?}", errors);
}
