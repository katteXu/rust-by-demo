use std::{fs::File, io::Write, path::Path};

static LOREM_IPSUM: &'static str =
    "Lorem ipsum dolor sit amet, consectetur adipisicing elit, sed do eiusmod
tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam,
quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo
consequat. Duis aute irure dolor in reprehenderit in voluptate velit esse
cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat cupidatat non
proident, sunt in culpa qui officia deserunt mollit anim id est laborum.
";

fn main() {
    let path = Path::new("out/lorem_ipsum.txt");
    let display = path.display();

    let mut file = match File::create(&path) {
        Ok(file) => file,
        Err(why) => panic!("couldn't create {}:{:?}", display, why),
    };

    match file.write_all(LOREM_IPSUM.as_bytes()) {
        Ok(_) => println!("successfully wrote to {}", display),
        Err(why) => println!("couldn't wirte to {}:{:?}", display, why),
    }
}
