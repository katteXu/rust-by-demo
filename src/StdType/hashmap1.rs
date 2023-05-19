use std::collections::HashMap;

fn call(number: &str) -> &str {
    match number {
        "798-1364" => "We're sorry",
        "645-7689" => "Hello this is Mr. Awesome's Pizza. My Name is Fred",
        _ => "Hi! Who is this again?",
    }
}

fn main() {
    let mut contacts = HashMap::new();

    contacts.insert("Daniel", "798-1364");
    contacts.insert("Ashley", "645-7689");
    contacts.insert("Katie", "435-8291");
    contacts.insert("Robert", "956-1745");

    match contacts.get("Daniel") {
        Some(&number) => println!("{}", number),
        _ => println!("Don't have Daniel'number."),
    }

    // 如果被插入的值为新内容，那么`HashMap::insert()` 返回None,
    // 否则返回`Some(value)`
    let a = contacts.insert("Daniel", "164-6743");

    match contacts.get("Ashley") {
        Some(&number) => println!("{}", call(number)),
        _ => println!("Don't have Ashly number."),
    }

    contacts.remove("Ashley");

    // 更新或者插入
    contacts
        .entry("Katie1")
        .and_modify(|e| *e = "abc")
        .or_insert("default");
    // println!("{:?}", a);

    println!("{:?}", contacts);

    for (contact, number) in contacts.iter() {
        println!("Calling {}: {}", contact, call(number));
    }
}
