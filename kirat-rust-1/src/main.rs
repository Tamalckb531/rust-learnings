fn main() {
    let s1: String = String::from("hello");
    println!("{} ", s1);
    let s2: String = s1;
    println!("{} ", s2);

    let a = 23;
    println!("{} ", a);
    let b = a;
    println!("{} ", b);
    println!("{} ", a);
}
