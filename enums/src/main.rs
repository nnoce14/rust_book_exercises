
fn main() {

    let home = IpAddr::V4(127, 0, 0, 1);

    let loopback = IpAddr::V4(String::from("::1"));

    let message = Message::Write(String::from("hello"));
    message.call();

    let some_number = Some(5);
    let some_string = Some("a string");

    let absent_number: Option<i32> = None;
}

enum IpAddr {
    V4(u8, u8, u8, u8);
    V6(String),
}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn call(&self) {
        // method body goes here
    }
}

