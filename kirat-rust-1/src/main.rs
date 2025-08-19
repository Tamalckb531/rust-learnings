fn main() {
    let a = 10;
    let b = 10;
    let mut sum = 5;
    sum += do_sum(a, b);
    println!("Su of {} and {} with 5 additional is {}", a, b, sum);
}

fn do_sum(a: i32, b: i32) -> i32 {
    return a + b;
}
