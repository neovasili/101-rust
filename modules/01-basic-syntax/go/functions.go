package main

import (
	"os"
	"fmt"
)

func is_divisible_by(number uint32, divisible_by uint32) bool {
	if divisible_by == 0 {
		return false
	}
	return number % divisible_by == 0
}

func fizzbuzz(number uint32) {
	divisible_by_3 := is_divisible_by(number, 3)
	divisible_by_5 := is_divisible_by(number, 5)

	if divisible_by_3 && divisible_by_5 { fmt.Println("fizzbuzz") }
	if divisible_by_3 && !divisible_by_5 { fmt.Println("fizz") }
	if !divisible_by_3 && divisible_by_5 { fmt.Println("buzz") }
	if !divisible_by_3 && !divisible_by_5 { fmt.Println(number) }
}

func fizzbuzz_to(number uint32) {

	for index := 1; index <= int(number); index++ {
		fizzbuzz(uint32(index))
	}
}

// Methods
type Rectangle struct {
	width uint32;
	height uint32;
}

func (rectagle *Rectangle) area() uint32 {
	return rectagle.width * rectagle.height
}

func (rectagle *Rectangle) inc_width(delta uint32) {
	rectagle.width += delta
}

func pick_one(first_element any, second_element any) any {
	if os.Getpid() % 2 == 0 { return first_element } else { return second_element }
}

// Overloading
func Test_overloading() {
	fmt.Println("Overloading")

	fmt.Printf("coin toss: %s\n", pick_one("heads", "tails"))
	fmt.Printf("cash prize: %d\n", pick_one(500, 1000))
}

func main() {
	fmt.Println("------------------------\n")
	// FizzBuzz
	fmt.Println("FizzBuzz")
	fizzbuzz_to(10)

	fmt.Println("------------------------\n")
	// Methods
  fmt.Println("Methods")

  var rectangle = Rectangle{ width: 10, height: 5 }
  fmt.Printf("old area: %d\n", rectangle.area())
  rectangle.inc_width(5)
  fmt.Printf("new area: %d\n", rectangle.area())

	fmt.Println("------------------------\n")
	// Overloading
	fmt.Println("Overloading")

	fmt.Printf("coin toss: %s\n", pick_one("heads", "tails"))
	fmt.Printf("cash prize: %d\n", pick_one(500, 1000))
}
