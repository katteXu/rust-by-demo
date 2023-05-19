/**
 * 动态数组 Vecotr
 */

fn main() {
    let mut collected_iterator: Vec<_> = (0..10).collect();

    let mut xs = vec![1i32, 2, 3];

    xs.push(43);
    collected_iterator.push(12);

    let len = xs.len();

    let first = xs.first();

    let pop = xs.pop();

    for x in xs.iter() {
        println!("{} ,{:?}", x, pop);
    }

    for x in xs.iter_mut() {
        *x = 2 * *x;
    }

    for (i, x) in xs.iter().enumerate() {
        println!("{} {}", i, x);
    }

    println!("{:?}", xs);
    // println!("len :{} pop :{} first :{}", len, pop.unwrap(), first);
}
