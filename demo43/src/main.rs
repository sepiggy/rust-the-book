// fn main() {
//     let s = String::from("hello world");
// 
//     let hello = &s[0..5];
//     let world = &s[6..11];
// 
//     println!("{}, {}", hello, world);
// }
//

// fn main() {
//     let s = String::from("hello");
//     let len = s.len();
//     let s11 = &s[0..2];
//     let s12 = &s[..2];
//     let s21 = &s[3..len];
//     let s22 = &s[3..];
//     let s31 = &s[0..len];
//     let s32 = &s[..];
// 
//     println!("{}", s11);
//     println!("{}", s12);
//     println!("{}", s21);
//     println!("{}", s22);
//     println!("{}", s31);
//     println!("{}", s32);
// }
//
fn main() {
    fn find_word(s: &String) -> &str {
        let bytes = s.as_bytes();

        for (i, &item) in bytes.iter().enumerate() {
            if item == b' ' {
                return &s[..i];
            }
        }
        &s[..]
    }

    let s = String::from("hello world");
    let result = find_word(&s);
    println!("{}", result);
}
