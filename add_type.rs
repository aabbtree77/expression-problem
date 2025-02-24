// Define a trait with both operations
trait Shape {
    fn area(&self) -> f64;
    fn perimeter(&self) -> f64;
}

// Existing types (original code)
// --------------------------------------------
struct Circle { radius: f64 }
impl Shape for Circle {
    fn area(&self) -> f64 { std::f64::consts::PI * self.radius.powi(2) }
    fn perimeter(&self) -> f64 { 2.0 * std::f64::consts::PI * self.radius }
}

struct Rectangle { width: f64, height: f64 }
impl Shape for Rectangle {
    fn area(&self) -> f64 { self.width * self.height }
    fn perimeter(&self) -> f64 { 2.0 * (self.width + self.height) }
}

// New data type: Triangle (added later)
// --------------------------------------------
struct Triangle { base: f64, height: f64 }
impl Shape for Triangle {
    fn area(&self) -> f64 { 0.5 * self.base * self.height }
    fn perimeter(&self) -> f64 {
        let hypotenuse = (self.base / 2.0).hypot(self.height);
        self.base + 2.0 * hypotenuse
    }
}

// Usage
// --------------------------------------------
fn main() {
    let shapes: Vec<Box<dyn Shape>> = vec![
        Box::new(Circle { radius: 3.0 }),
        Box::new(Rectangle { width: 4.0, height: 5.0 }),
        Box::new(Triangle { base: 6.0, height: 4.0 }), // New type added
    ];

    for shape in shapes {
        println!(
            "Area: {:.2}, Perimeter: {:.2}",
            shape.area(),
            shape.perimeter()
        );
    }
}
