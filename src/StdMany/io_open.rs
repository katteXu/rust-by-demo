use std::{fs::File, io::Read, path::Path};

fn main() {
    let path = Path::new("hello.txt");
    let display = path.display();
    // let debug = pat

    let mut file = match File::open(&path) {
        Ok(file) => file,
        Err(why) => panic!("couldn't open {}, :{:?}", display, why),
    };

    let mut s = String::new();

    match file.read_to_string(&mut s) {
        Ok(_) => print!("{} contains: \n{}", display, s),
        Err(why) => panic!("couldn't read {} :{:?}", display, why),
    }
}
