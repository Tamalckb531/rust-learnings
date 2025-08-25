enum Shape {
    Circle(f64),         //? For the radius
    Square(f64),         //? For the length
    Rectangle(f64, f64), //? For the width and height
}
fn main() {
    let circle = Shape::Circle(5.0);
    let square = Shape::Square(4.0);
    let rectangle = Shape::Rectangle(3.0, 6.0);

    calculate_area(circle);
    calculate_area(square);
    calculate_area(rectangle);
}

fn calculate_area(shape: Shape) -> f64 {
    return 0.2;
}
