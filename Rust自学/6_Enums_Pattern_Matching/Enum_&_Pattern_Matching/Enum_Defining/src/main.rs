enum IpAddrKind{
    V4(u8, u8, u8, u8),
    V6(String),
}

#[derive(Debug)]
enum Message{
    Quit,
    Move {x: i32, y: i32},
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message{
    fn call(&self){
        println!("The message reads {:#?}", self);
    }
}

// enum Option<T>{
//     None,
//     Some(T),
// }

fn main() {
    let four = IpAddrKind::V4(127, 0, 0, 1);
    let six = IpAddrKind::V6(String::from("::1"));

    let msg1 = Message::Quit;

    let msg2 = Message::Move{
        x: 1,
        y: -1,
    };

    let msg3 = Message::Write(String::from("localhost"));

    let msg4 = Message::ChangeColor(100, 101, 99);

    msg1.call();
    msg2.call();
    msg3.call();
    msg4.call();

    let some_number = Some(5);
    let some_string = Some("a string");
    let absent_var: Option<i32> = None;

    // let x: i8 = 5;
    // let y: Option<i8> = Some(5);

    // let sum = x + y;
}
