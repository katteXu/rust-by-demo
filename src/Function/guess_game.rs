use rand::Rng;
use std::io;

fn main() {
    let secret_number = rand::thread_rng().gen_range(1..101);

    loop {
        println!("请输入一个 1 到 100 的整数：");

        let mut guess = String::new();

        io::stdin().read_line(&mut guess).expect("读取行失败");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("你猜的数字是：{}", guess);

        match guess.cmp(&secret_number) {
            std::cmp::Ordering::Less => println!("太小了！"),
            std::cmp::Ordering::Greater => println!("太大了！"),
            std::cmp::Ordering::Equal => {
                println!("恭喜你，猜对了！");
                break;
            }
        }
    }
}
