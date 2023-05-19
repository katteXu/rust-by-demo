use std::collections::HashSet;

fn main() {
    let mut a: HashSet<i32> = vec![1i32, 2, 3].into_iter().collect();
    let mut b: HashSet<i32> = vec![2i32, 3, 4].into_iter().collect();
    assert!(a.insert(4));
    assert!(a.contains(&4));

    // 如果值已经存在，那么 `HashSet::insert()` 返回 false。
    // assert!(b.insert(4), "Value 4 is already in set B!");
    // 改正 ^ 将此行注释掉。

    b.insert(5);

    println!("A:{:?}", a);
    println!("B:{:?}", b);

    // 并集
    println!("Union: {:?}", a.union(&b).collect::<Vec<&i32>>());
    // a 对b的差集
    println!("Difference: {:?}", a.difference(&b).collect::<Vec<&i32>>());
    // 交集
    println!(
        "Intersection: {:?}",
        a.intersection(&b).collect::<Vec<&i32>>()
    );

    println!(
        "Symmetric difference: {:?}",
        a.symmetric_difference(&b).collect::<Vec<&i32>>()
    )
}
