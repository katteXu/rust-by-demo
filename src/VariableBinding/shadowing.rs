fn main() {
    let long_lived_binding = 1;

    {
        let short_lived_binding = 2;

        println!("{}", short_lived_binding);

        let long_lived_binding = 5_f32;
        let long_lived_binding2 = 5f32;

        println!("long1 {},long2 {}", long_lived_binding, long_lived_binding2);
    }

    // 报错！ short lived binding 在此作用域不存在
    // println!("{}",shor_lived_binding);
    println!("OUT {}", long_lived_binding);

    let long_lived_binding = 'a';

    println!("OUT {}", long_lived_binding);
}
