fn next_birthday(current_age: Option<u8>) -> Option<String> {
    let next_age = current_age?;

    Some(format!("Next year I will be {}", next_age))
}

struct Person {
    job: Option<Job>,
}

#[derive(Clone, Copy)]
struct Job {
    phone_number: Option<PhoneNumber>,
}

#[derive(Clone, Copy)]
struct PhoneNumber {
    area_code: Option<u8>,
    number: i32,
}

impl Person {
    fn work_phone_area_code(&self) -> Option<u8> {
        self.job?.phone_number?.area_code
    }
}

#[derive(Debug)]
enum Food {
    Apple,
    Carrot,
    Potato,
}

struct Peeled(Food);
struct Chopped(Food);
#[derive(Debug)]
struct Cooked(Food);

fn process(food: Option<Food>) -> Option<Cooked> {
    food.map(|f| Peeled(f))
        .map(|Peeled(f)| Chopped(f))
        .map(|Chopped(f)| Cooked(f))
}

fn process2(food:Option<Food>) ->Option<Cooked> {
    let result:Option<Peeled> = food.and_then(|f| Some(Peeled(f)));

    Some(Cooked(Food::Apple))
}
fn main() {
    let p = Person {
        job: Some(Job {
            phone_number: Some(PhoneNumber {
                area_code: Some(86),
                number: 88288844,
            }),
        }),
    };

    let area_code = p.work_phone_area_code();

    println!("Area Code: {:?}", area_code);

    let food = Some(Food::Apple);

    let result = process(food);
    println!("{:?}", result);
}
