// enum IpAddrKind{
//     V4,
//     V6,
// }
//
// #[derive(Debug)]
// enum IpAddr {
//     V4(String),
//     V6(String),
// }
//
// fn main() {
//     let home = IpAddr::V4(String::from("127.0.0.1"));
//     let loopback = IpAddr::V6(String::from("::1"));
//
//     println!("{:#?}", home);
//     println!("{:#?}", loopback);
// }

// #[derive(Debug)]
// enum IpAddr {
//     v4(u8, u8, u8, u8),
//     v6(String),
// }
//

// #[derive(Debug)]
// enum Message {
//     Quit,
//     Move { x: i32, y: i32 },
//     Write(String),
//     ChangeColor(i32, i32, i32),
// }
// 
// impl Message {
//     fn call(&self) {
//         println!("{:#?}", self);
//     }
// }
// 
// fn main() {
//     let m = Message::Write(String::from("hello"));
//     m.call();
// }

fn main() {
	let some_number = Some(5);
	let some_string = Some("a_string");
	let absent_number: Option<i32> = None;

	println!("{:#?}", some_number);
	println!("{:#?}", some_string);
	println!("{:#?}", absent_number);
}
