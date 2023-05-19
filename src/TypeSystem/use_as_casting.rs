/**
 * 原生类型的类型转化 (cast)
 * 指定字面量类型
 * 使用类型判断 (type inference)
 * 给类型起别名 (alias)
 */

// 不显示类型之间转换产生的溢出警告
#[allow(overflowing_literals)]

fn main() {
    let decimal = 65.4321_f32;

    let integer: u32 = decimal as u32;

    println!("{}", integer);

    let integer: u8 = decimal as u8;
    let character: char = integer as char;

    println!("{}", character);

    println!("casting {}->{}->{}", decimal, integer, character);

    // 1000 as u8
    println!("1000 to  u8 :{}", 1000 % 256);

    println!("-1 as u8 {}",(-1i8) as u8);



    println!("232 as i8 {}" , (232 as i8) as u8);


}
