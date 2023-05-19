use std::{
    io::{Read, Write},
    process::{Command, Stdio},
};

static PANGRAM: &'static str = "the quick brown fox jumped over the lazy dog\n";
fn main() {
    let process = match Command::new("wc")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
    {
        Ok(process) => process,
        Err(why) => panic!("couldn't spawn wc: {:?}", why),
    };

    match process.stdin.unwrap().write_all(PANGRAM.as_bytes()) {
        Ok(_) => println!("sent pangram to wc"),
        Err(why) => panic!("couldn't write to wc stdin: {:?}", why),
    }

    let mut s = String::new();

    match process.stdout.unwrap().read_to_string(&mut s) {
        Ok(_) => println!("wc responded with :\n {}", s),
        Err(why) => panic!("couldn't read from wc stdout: {:?}", why),
    };
}
