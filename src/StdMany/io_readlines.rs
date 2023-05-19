use std::{
    fs::File,
    io::{self, BufRead},
    path::Path,
};

fn main() {
    if let Ok(lines) = read_lines("out/lorem_ipsum.txt") {
        for line in lines {
            if let Ok(line) = line {
                println!("{}", line);
            }
        }
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
