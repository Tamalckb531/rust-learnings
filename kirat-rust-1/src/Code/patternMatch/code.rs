enum Shape {
    Circle(f64),         //? For the radius
    Square(f64),         //? For the length
    Rectangle(f64, f64), //? For the width and height
}
fn main() {
    let circle = Shape::Circle(5.0);
    let square = Shape::Square(4.0);
    let rectangle = Shape::Rectangle(3.0, 6.0);

    let circle_area = calculate_area(circle);
    let square_area = calculate_area(square);
    let rect_area = calculate_area(rectangle);

    println!(
        "Area of circle: {}, square: {}, rectangle: {}",
        circle_area, square_area, rect_area
    );
}

fn calculate_area(shape: Shape) -> f64 {
    let ans = match shape {
        Shape::Circle(radius) => 3.14 * radius * radius,
        Shape::Rectangle(width, height) => width * height,
        Shape::Square(side) => side * side,
    };

    return ans;
}
