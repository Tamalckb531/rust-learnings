//? Generics
struct Point<I, S> {
    x: I,
    y: S,
    z: S,
}
fn main() {
    let integer_point = Point {
        x: 5,
        y: "10",
        z: "15",
    };
    let float_point = Point {
        x: 5.5,
        y: "10.2",
        z: "15.3",
    };

    println!(
        "Integer Point: ({}, {}, {})",
        integer_point.x, integer_point.y, integer_point.z
    );
    println!(
        "Float Point: ({}, {}, {})",
        float_point.x, float_point.y, float_point.z
    );
}

//? Error handling
use std::fs;

fn main() {
    let res = fs::read_to_string("example.txt");
    match res {
        Ok(content) => {
            println!("File content: {}", content);
        }
        Err(err) => {
            println!("Error: {}", err);
        }
    }
}
