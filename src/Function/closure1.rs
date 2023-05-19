/**
 * closure
 */

fn main() {
    use std::mem;

    let color = String::from("red");
    println!("{}", color);
    let print = || {
        let a = color.clone();
    };

    print();

    let _color_moved = color;
    let mut count = 0;
    let mut inc = || {
        count += 1;
        println!("count :{}", count);
    };

    inc();

    // let _reborrow = &count;

    inc();

    let _count_reborrow = &count;

    let moveable = Box::new(3);

    let consume = || {
        println!("{}", moveable);
        // mem::drop(moveable)
    };

    consume();

    consume();

    let haystack = vec![1, 2, 3];

    let contains = move |needle| haystack.contains(needle);

    println!("haystack : {:?}", contains(&1));
    println!("haystack : {:?}", contains(&4));

    // println!("There're {} elements in vec", haystack.len());
}
