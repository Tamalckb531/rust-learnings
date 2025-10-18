//! Normal Lifetime generic code
// fn longest<'a>(a: &'a str, b: &'a str) -> &'a str {
//     if a.len() > b.len() {
//         return a;
//     } else {
//         return b;
//     }
// }
// fn main() {
//     let longest_str;
//     let str1 = String::from("small");
//     {
//         let str2 = String::from("longer");
//         longest_str = longest(&str1, &str2);
//         println!("{}", longest_str);
//     }
// }

//! Struct with Lifetime generic code
struct User<'a> {
    name: &'a str,
}

fn main() {
    let name = String::from("Tamal");
    let user = User { name: &name };
    println!("{}", user.name);
}
