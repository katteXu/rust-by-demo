#[derive(Debug)]
struct EvenNumber(i32);

impl TryFrom<i32> for EvenNumber {
    type Error = ();

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        if value % 2 == 0 {
            Ok(EvenNumber(value))
        } else {
            Err(())
        }
    }
}

fn main() {
    let a = 28;

    let b = EvenNumber::try_from(a).unwrap_or(EvenNumber(0));

    println!("{:?}", b);
}
