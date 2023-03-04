package main

import (
	"fmt"
)

// References
func main() {
	fmt.Println("------------------------\n")

	var x int32 = 10;
	var ref_x *int32 = &x;
	*ref_x = 20;
	fmt.Printf("x: %d\n", x);
}
