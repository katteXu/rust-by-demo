fn main() {
    let number = 13;

    println!("Tell me about {}!", number);

    match number {
        1 => println!("One!"),
        2 | 3 | 5 | 7 | 11 => println!("This is a prime"),
        13..=19 => println!("A teen"),
        _ => println!("Ain't special"),
    }

    let boolean = true;

    let binary = match boolean {
        true => 1,
        false => 0,
    };

    println!("{} -> {}", boolean, binary);

    // 元祖
    let tuple = (0, 1, 2);

    println!("Tell me about {:?}", tuple);

    match tuple {
        (0, y, z) => println!("First is zero, `y` is {}, `z` is {}", y, z),
        (1, ..) => println!("First is 1 and the rest does't matter"),
        _ => println!("This is not a tuple"),
    }

    // 枚举
    enum Color {
        Red,
        Blue,
        Green,
        RGB(u8, u8, u8),
        HSV(u8, u8, u8),
        HSL(u8, u8, u8),
        CMY(u32, u32, u32),
    }

    let color = Color::RGB(122, 17, 40);

    println!("What color is it?");

    match color {
        Color::Red => println!("Red"),
        Color::Blue => println!("Blue"),
        Color::Green => println!("Green"),
        Color::RGB(r, g, b) => {
            println!("Red is {}, Green is {} Blue is {}", r, g, b)
        }
        Color::HSV(h, s, v) => {
            println!("Hue is {}, saturation is {}, value is {}", h, s, v)
        }
        Color::HSL(h, s, l) => {
            println!("Hue is {}, saturation is {}, lightness is {}", h, s, l)
        }
        Color::CMY(c, m, y) => {
            println!("Cyan is {}, magenta is {}, yellow is {}", c, m, y)
        }
    };

    // 指针和引用
    let reference = &4;
    let _not_a_reference = 3;
    let ref _is_reference = 3;
    let  value = 5;

    let mut mut_value = 5;


    match reference {
        &val => println!("Reference value is {}", val),
    }

    match *reference {
        val => println!("Reference value is {}", val),
    }

    match value {
        val => println!("Value is {:?}", val),
    }

    match mut_value {
        mut val => {
            val += 10;
            println!("Mut value is {:?}", val)
        },
    }
}
