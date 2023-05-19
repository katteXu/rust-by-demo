/**
 * Enum
 */

enum WebEvent {
    PageLoad,
    PageUnload,

    KeyPress(char),
    Paste(String),
    Click { x: i64, y: i64 },
}

fn inspect(evnet: WebEvent) {
    match evnet {
        WebEvent::PageLoad => println!("Page Load"),
        WebEvent::PageUnload => println!("Page Unload"),
        WebEvent::KeyPress(c) => println!("KeyPress: {:?}", c),
        WebEvent::Paste(s) => println!("Paste: \"{:?}\"", s),
        WebEvent::Click { x, y } => println!("Click: ({:?},{:?})", x, y),
    }
}

enum VeryVerboseEnumOfThingsToDoWithNumbers {
    Add,
    Subtract,
    Multiply,
    Divide,
}

type Operations = VeryVerboseEnumOfThingsToDoWithNumbers;

impl Operations {
    fn run(&self, x: i32, y: i32) -> i32 {
        match self {
            Self::Add => x + y,
            Self::Subtract => x - y,
            Self::Multiply => x * y,
            Self::Divide => x / y,
        }
    }
}

fn main() {
    let pressed = WebEvent::KeyPress('a');

    let pasted = WebEvent::Paste(String::from("my text"));
    let click = WebEvent::Click { x: 20, y: 90 };
    let load = WebEvent::PageLoad;
    let unload = WebEvent::PageUnload;

    inspect(pressed);
    inspect(pasted);
    inspect(click);
    inspect(load);
    inspect(unload);

    let add = Operations::Add;
}
