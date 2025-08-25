//? Borrowing
fn main() {
    let s1: String = String::from("hello");
    let s2 = &s1;
    println!("{} ", s1);
    println!("{} ", s2);
}

//? Multiple borrowing without mutability :

fn main() {
    let my_string = String::from("Hello, Rust");
    let s1 = &my_string;
    let s2 = &my_string;
    let s3 = &my_string;
    borrow_var(&my_string);
    println!("{}", s1);
    println!("{}", s2);
    println!("{}", s3);
    println!("{}", my_string);
}

fn borrow_var(some_string: &String) {
    println!("{}", some_string);
}

//? One mutable borrowing :

fn main() {
    let mut s1 = String::from("Hello, Rust");
    update_str(&mut s1);
    println!("{}", s1)
}

fn update_str(s: &mut String) {
    s.push_str(" Tamal");
}
