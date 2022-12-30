/* Enums */
enum IpAddrKind {
    V4(u8, u8, u8, u8),
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
        // method body would be defined here
    }
}

enum Option<T> {
    None,
    Some(T),
}

fn main() {
    chapter_6_1();
    chapter_6_2();
    chapter_6_3();
}

fn chapter_6_1() {
    println!("********* 6.1 *********");

    let home = IpAddrKind::V4(196, 128, 0, 1);
    let loopback = IpAddrKind::V6(String::from("::1"));

    let m = Message::Write(String::from("Hello!"));
    m.call();

    let some_number = Some(5);
    let some_char = Some('e');

    let absent_number: Option<i32> = None;
}

fn chapter_6_2() {
    println!("********* 6.2 *********");
}

fn chapter_6_3() {
    println!("********* 6.3 *********");
}