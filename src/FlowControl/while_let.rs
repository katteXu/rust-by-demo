fn main() {
    let mut optional = Some(0);

    while let Some(i) = optional {
        if i > 9 {
            println!("Greater than 9");
            optional = None;
        } else {
            println!("i is `{:?}.Try again`", i);
            optional = Some(i + 1)
        }
    }

    let mut optional = Some(0);
    loop {
        if let Some(i) = optional {
            if i > 9 {
                println!("Greater than 9");
                optional = None;
            } else {
                println!("i is `{:?}.Try again`", i);
                optional = Some(i + 1)
            }
        } else {
            break;
        }
    }
}
