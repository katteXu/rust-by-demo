use std::str;

/**
 * string 字符串
 */

fn main() {
    let pangram = "the quick brown fox jumps over the lazy dog";

    for word in pangram.split_whitespace().rev() {
        println!("word: {}", word);
    }

    let mut chars: Vec<char> = pangram.chars().collect();
    chars.sort();
    chars.dedup();

    println!("chars: {:?}", chars);

    let mut string = String::new();

    for c in chars {
        string.push(c);
        string.push_str(", ")
    }

    let chars_to_trim: &[char] = &[' ', ','];

    let str = string.trim_matches(chars_to_trim);
    println!("string: {}", string);
    println!("Used characters: {:?}", str);

    let alice = String::from("I like dogs");

    let bob = alice.replace("dog", "cat");

    println!("alice: {}", alice);
    println!("bob: {}", bob);

    // byte

    let byte_escape = "\x52\x75\x73\x74! \u{211D}";
    // let long_string = "String literals
    // can span multiple lines.
    // The linebreak and indentation here ->\
    // <- can be escaped too!";
    let long_string = "String literals
                        can span multiple lines.
                        The linebreak and indentation here ->\
                        <- can be escaped too!";
    println!("byte: {}", byte_escape);
    println!("{}", long_string);

    let raw_str = r"Escapes don't work here: \x3F \u{211D}";
    println!("{}", raw_str);

    // 如果你要在原始字符串中写引号，请在两边加一对 #
    let quotes = r#"And then I said: "There is no escape!""#;
    println!("{}", quotes);

    // 如果字符串中需要写 "#，那就在定界符中使用更多的 #。
    // 可使用的 # 的数目没有限制。
    let longer_delimiter = r###"A string with "# in it. And even "##!"###;
    println!("{}", longer_delimiter);

    let bytestring = b"this is a bytestring";

    println!("A bytestring :{:?}", bytestring);

    let escaped = b"\x52\x75\x73\x74 as bytes";
    // ...但不能使用 Unicode 转义字符
    // let escaped = b"\u{211D} is not allowed";
    println!("Some escaped bytes: {:?}", escaped);

    // 原始字节串和原始字符串的写法一样
    let raw_bytestring = br"\u{211D} is not escaped here";
    println!("{:?}", raw_bytestring);

    // 把字节串转换为 &str 可能失败
    if let Ok(my_str) = str::from_utf8(raw_bytestring) {
        println!("And the same as text: '{}'", my_str);
    }

    let shift_jis = b"\x82\xe6\x82\xa8\x82\xb1\x82";
    println!("shift_jis :{:?}", shift_jis);

    // 但这样的话它们就无法转换成 &str 了
    match str::from_utf8(shift_jis) {
        Ok(my_str) => println!("Conversion successful: '{}'", my_str),
        Err(e) => println!("Conversion failed: {:?}", e),
    };
    
}
