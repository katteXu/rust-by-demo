fn main() {
    // if_else();
    // my_loop();
    // my_loop2();
    // return_loop();
    // my_while();
    my_for();
}

/**
 * if else
 */
fn if_else() {
    let n = -11;

    if n < 0 {
        print!("{} is negative", n);
    } else if n > 0 {
        print!("{} is positive", n);
    } else {
        print!("{} is zero", n);
    }

    let big_n = if n < 10 && n > -10 {
        println!(", and is a small number ,increased ten-fold");
        10 * n
    } else {
        println!(", and is a big number,half the number");
        n / 2
    };

    println!("{} -> {}", n, big_n);
}

/**
 * loop
 */
fn my_loop() {
    let mut count: u8 = 0;

    println!("Let's count until infinity!");

    loop {
        count += 1;
        if count == 3 {
            println!("three");

            continue;
        }

        println!("{}", count);

        if count == 5 {
            println!("OK that's enough");
            // break;
        }
    }
}

fn my_loop2() {
    'outer: loop {
        println!("Entered the outer loop");

        'inner: loop {
            println!("Entered the inner loop");
            break 'outer;
            break 'inner;
        }

        println!("THis point will never be reached");
    }

    println!("Exited the outer loop");
}

fn return_loop() {
    let mut counter = 1;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("{result}");
}

fn my_while() {
    let mut vec = Vec::new();
    // 2 1 0 2 0 1 2 2 1 0 2 1 2 2 1 0
    let mut n = 1;
    let mut count = 0;
    while n < 101 {
        if n % 15 == 0 {
            println!("fizzbuzz {}", count);
            vec.push(count);
            count = 0;
        } else if n % 5 == 0 {
            println!("buzz {}", count);
            vec.push(count);
            count = 0;
        } else if n % 3 == 0 {
            println!("fizz {}", count);
            vec.push(count);
            count = 0;
        } else {
            count += 1;
            print!("{} ,", n);
        }

        n += 1;
    }

    println!("{:?}", vec);
}


fn my_for(){
    for n in 1..=100 {
        println!("{}", n);
    }
}