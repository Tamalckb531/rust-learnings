struct User {
    name: String,
    age: u8,
    email: String,
    active: bool,
}
fn main() {
    let name = String::from("Tamal");
    let email = String::from("ckbtamaldipnew@gmail.com");
    let user = User {
        name: name,
        age: 21,
        email: email,
        active: true,
    };

    println!(
        "{} with email {} is {} years old and currently active {}",
        user.name, user.email, user.age, user.active
    );
}
