fn call_me<F>(f: F)
where
    F: Fn(),
{
}

fn function() {
    println!("I'm a function!");
}

fn main() {
    let closure = || println!("I'm a closure!");

    call_me(closure);
    call_me(function);
}
