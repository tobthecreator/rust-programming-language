#[derive(Debug)]
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

// Impl can also be written for enums
impl Message {
    fn call(&self) {
        println!("{:?}", self);
        // method body would be defined here
    }
}

fn main() {
    println!("Hello, world!");

    let m = Message::Write(String::from("hello"));
    m.call();
}
