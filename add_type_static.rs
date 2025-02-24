// ==== Define Operations as Traits ====
trait Area {
    fn area(&self) -> f64;
}

trait Perimeter {
    fn perimeter(&self) -> f64;
}

// ==== Define Data Types (Open for Extension) ====
struct Circle { radius: f64 }
struct Rectangle { width: f64, height: f64 }
struct Triangle { base: f64, height: f64 } // Added later

// ==== Implement Operations for Types ====
impl Area for Circle {
    fn area(&self) -> f64 { std::f64::consts::PI * self.radius.powi(2) }
}
impl Perimeter for Circle {
    fn perimeter(&self) -> f64 { 2.0 * std::f64::consts::PI * self.radius }
}

impl Area for Rectangle {
    fn area(&self) -> f64 { self.width * self.height }
}
impl Perimeter for Rectangle {
    fn perimeter(&self) -> f64 { 2.0 * (self.width + self.height) }
}

impl Area for Triangle {
    fn area(&self) -> f64 { 0.5 * self.base * self.height }
}
impl Perimeter for Triangle {
    fn perimeter(&self) -> f64 {
        let hypotenuse = (self.base / 2.0).hypot(self.height);
        self.base + 2.0 * hypotenuse
    }
}

// ==== Generic Function for Static Dispatch ====
fn process_shape<S: Area + Perimeter>(shape: &S) {
    println!(
        "Area: {:.2}, Perimeter: {:.2}",
        shape.area(),
        shape.perimeter()
    );
}

// ==== Usage (No Heterogeneous Collections) ====
fn main() {
    let circle = Circle { radius: 3.0 };
    let rectangle = Rectangle { width: 4.0, height: 5.0 };
    let triangle = Triangle { base: 6.0, height: 4.0 };

    process_shape(&circle);
    process_shape(&rectangle);
    process_shape(&triangle);
}
