
enum IpAddr1 {
    V4(String),
    V6(String),
}

// enums allow for 
enum IpAddr2 {
    V4(u8, u8, u8, u8),
    V6(String),
}

enum Message {
    _Quit,
    _Move { x: i32, y: i32 },
    Write(String),
    _ChangeColor(i32, i32, i32),
}

// Option is Rust's way of dealing with null, none, void values and it's included in prelude so it's always in scope
// 
// enum Option<T> {     // <T> is a generic type parameter meaning it can hold any type
//    Some(T),          // like e.g let some_string = Some("a string");
//    None,             // if None is used it's type needs to be specified e.g. let absent_number: Option<i32> = None;
//}

impl Message {
    fn call(&self) {
        // method body would be defined here
    }
}

// struct IpAddr {
//     kind: IpAdrKd,
//     address: String,
// }

fn main() {
    // let home = IpAddr {
    //    kind = IpAdrKd::V4, // Instatiated from enum and namespaced under identifier V4
    //    address = String::from("120.0.0.1"),
    // };
    
    // This is equal to the above block but no struct is needed.
    let _home = IpAddr1::V4(String::from("127.0.0.1"));
    let _loopback = IpAddr1::V6(String::from("::1"));

    // can have different types and more flexible amounts of associated data.
    let _home2 = IpAddr2::V4(127, 0, 0, 1);
    let _loopback2 = IpAddr2::V6(String::from("::1"));

    // since this is ver common it is included in https://doc.rust-lang.org/std/net/enum.IpAddr.html
    // std::net::IpAddr

    //m that has the value Message::Write(String::from("hello")
    let message = Message::Write(String::from("hello"));
    // that is what self will be in the body of the call method when m.call() runs
    message.call();

    // handling null vs. automagic checking
    let x: i8 = 5;
    let y: Option<i8> = Some(5);

    let sum = x + y; // this errors so you make sure to handle null yourself
}

fn _route(_ip_kind: IpAddr1) {
    // implement some routing
}