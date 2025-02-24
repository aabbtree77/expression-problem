// ==== Macro-Based Solution ====
macro_rules! define_shapes {
    // Define supported shapes and operations
    ($($Shape:ident),*) => {
        // Generate an enum wrapping all shapes
        #[derive(Clone)]
        enum Shape {
            $($Shape($Shape)),*
        }

        // Generate trait implementations for each operation
        macro_rules! impl_operation {
            ($Trait:ident, $method:ident) => {
                impl $Trait for Shape {
                    fn $method(&self) -> f64 {
                        match self {
                            $(Shape::$Shape(s) => s.$method()),*
                        }
                    }
                }
            };
        }

        // Define operations as traits
        trait Area { fn area(&self) -> f64; }
        trait Perimeter { fn perimeter(&self) -> f64; }

        // Implement operations for the enum
        impl_operation!(Area, area);
        impl_operation!(Perimeter, perimeter);
    };
}

// ==== User Code (Extensible) ====
// Define concrete types
#[derive(Clone)]
struct Circle { radius: f64 }
#[derive(Clone)]
struct Rectangle { width: f64, height: f64 }
#[derive(Clone)] // Added later
struct Triangle { base: f64, height: f64 }

// Implement operations for each type
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

// Invoke macro to generate enum and operations (add new types here)
define_shapes!(Circle, Rectangle, Triangle);

// ==== Usage ====
fn main() {
    let shapes = vec![
        Shape::Circle(Circle { radius: 3.0 }),
        Shape::Rectangle(Rectangle { width: 4.0, height: 5.0 }),
        Shape::Triangle(Triangle { base: 6.0, height: 4.0 }), // New type
    ];

    for shape in shapes {
        println!(
            "Area: {:.2}, Perimeter: {:.2}",
            shape.area(),
            shape.perimeter()
        );
    }
}
