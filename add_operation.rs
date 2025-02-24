// Define a closed enum (fixed variants)
enum Shape {
    Circle(f64),
    Rectangle(f64, f64),
    Triangle(f64, f64), // Included upfront (closed)
}

// Existing operation: area
fn area(shape: &Shape) -> f64 {
    match shape {
        Shape::Circle(r) => std::f64::consts::PI * r * r,
        Shape::Rectangle(w, h) => w * h,
        Shape::Triangle(b, h) => 0.5 * b * h,
    }
}

// New operation: perimeter (added later)
fn perimeter(shape: &Shape) -> f64 {
    match shape {
        Shape::Circle(r) => 2.0 * std::f64::consts::PI * r,
        Shape::Rectangle(w, h) => 2.0 * (w + h),
        Shape::Triangle(b, h) => {
            let hypotenuse = (b / 2.0).hypot(*h);
            b + 2.0 * hypotenuse
        }
    }
}

fn main() {
    let shapes = vec![
        Shape::Circle(3.0),
        Shape::Rectangle(4.0, 5.0),
        Shape::Triangle(6.0, 4.0),
    ];

    for shape in &shapes {
        println!(
            "Area: {:.2}, Perimeter: {:.2}",
            area(shape),
            perimeter(shape) // New operation used
        );
    }
}
