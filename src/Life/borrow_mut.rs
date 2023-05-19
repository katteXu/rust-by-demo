// #[derive(Clone, Copy)]
#[derive(Debug, Clone)]
struct Book {
    author: &'static str,
    title: &'static str,
    year: u32,
}

fn borrow_book(book: &Book) {}

fn new_edition(book: &mut Book) {
    book.year = 2014;
}
fn main() {
    let immutabook = Book {
        author: "DH",
        title: "TT",
        year: 1988,
    };

    let mut mutbook = immutabook.clone();

    new_edition(&mut mutbook);
    println!("{:#?}, {:#?}", mutbook, immutabook);
}
