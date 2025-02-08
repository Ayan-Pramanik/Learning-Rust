enum Shape {
    Circle(f64),
    Square(f64),
    Rectangle(f64, f64),
}

fn calculate_area(shape: Shape) -> f64 {
    let area = match shape {
        Shape::Circle(r) => 3.14 * r * r,
        Shape::Square(s) => s * s,
        Shape::Rectangle(l,b) => l * b,
    };
    return area;
}

fn calulate_perimeter(shape: Shape) -> f64 {
    match shape {
        Shape::Circle(r) => 2.0 * 3.14 * r,
        Shape::Square(s) => 4.0 * s,
        Shape::Rectangle(l,b) => 2.0 * (l + b),
    }
}

fn main() {
    let rect = Shape::Rectangle(1.0,2.0);
    let circle = Shape::Circle(3.0);
    println!("{}",calculate_area(rect));
    println!("{}",calculate_area(circle));
}