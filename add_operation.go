package main

import (
	"fmt"
	"math"
)

// Existing interfaces and types
// --------------------------------------------
type AreaCalculator interface{ Area() float64 }

type Circle struct{ Radius float64 }
func (c Circle) Area() float64 { return math.Pi * c.Radius * c.Radius }

type Rectangle struct{ Width, Height float64 }
func (r Rectangle) Area() float64 { return r.Width * r.Height }

type Triangle struct{ Base, Height float64 }
func (t Triangle) Area() float64 { return 0.5 * t.Base * t.Height }

// New operation: Perimeter
// --------------------------------------------
type PerimeterCalculator interface{ Perimeter() float64 }

// Combined interface for both operations
type ShapeOperations interface {
	AreaCalculator
	PerimeterCalculator
}

// Wrappers to add Perimeter to existing types
type CircleWrapper struct{ Circle }
func (cw CircleWrapper) Perimeter() float64 { return 2 * math.Pi * cw.Radius }

type RectangleWrapper struct{ Rectangle }
func (rw RectangleWrapper) Perimeter() float64 { return 2 * (rw.Width + rw.Height) }

type TriangleWrapper struct{ Triangle }
func (tw TriangleWrapper) Perimeter() float64 {
	hypotenuse := math.Hypot(tw.Base/2, tw.Height)
	return tw.Base + 2*hypotenuse
}

// Usage
// --------------------------------------------
func main() {
	// Original shapes
	circle := Circle{Radius: 3}
	rectangle := Rectangle{Width: 4, Height: 5}
	triangle := Triangle{Base: 6, Height: 4}

	// Wrap shapes to support both Area and Perimeter
	wrappedShapes := []ShapeOperations{
		CircleWrapper{circle},      // CircleWrapper embeds Circle
		RectangleWrapper{rectangle}, // RectangleWrapper embeds Rectangle
		TriangleWrapper{triangle},   // TriangleWrapper embeds Triangle
	}

	// Loop through shapes and print both operations
	for _, shape := range wrappedShapes {
		// Access Area() and Perimeter() directly from the wrapper
		fmt.Printf(
			"Area: %.2f, Perimeter: %.2f\n",
			shape.Area(),      // From embedded original type (Circle/Rectangle/Triangle)
			shape.Perimeter(), // From wrapper
		)
	}
}
