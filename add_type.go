package main

import (
	"fmt"
	"math"
)

// Single interface combining Area and Perimeter
type ShapeOperations interface {
	Area() float64
	Perimeter() float64
}

// Existing shapes implementing ShapeOperations directly
// ---------------------------------------------------
type Circle struct{ Radius float64 }
func (c Circle) Area() float64      { return math.Pi * c.Radius * c.Radius }
func (c Circle) Perimeter() float64 { return 2 * math.Pi * c.Radius }

type Rectangle struct{ Width, Height float64 }
func (r Rectangle) Area() float64 { return r.Width * r.Height }
func (r Rectangle) Perimeter() float64 { return 2 * (r.Width + r.Height) }

// New shape added later
// ---------------------------------------------------
type Triangle struct{ Base, Height float64 }
func (t Triangle) Area() float64 { return 0.5 * t.Base * t.Height }
func (t Triangle) Perimeter() float64 {
	equalSide := math.Hypot(t.Base/2, t.Height) // For an isosceles triangle
	return t.Base + 2*equalSide
}

// Usage
// ---------------------------------------------------
func main() {
	shapes := []ShapeOperations{
		Circle{Radius: 3},                  // Original shape
		Rectangle{Width: 4, Height: 5},     // Original shape
		Triangle{Base: 6, Height: 4},       // New shape
	}

	for _, shape := range shapes {
		fmt.Printf(
			"Area: %.2f, Perimeter: %.2f\n",
			shape.Area(),
			shape.Perimeter(),
		)
	}
}
