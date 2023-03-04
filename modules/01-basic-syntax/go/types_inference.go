package main

import (
	"fmt"
)

func takes_u32(x uint32) {
  fmt.Printf("u32: %d\n", x)
}

func takes_i8(y int8) {
  fmt.Printf("i8: %d\n", y)
}

// Types inference
func main() {
  fmt.Println("------------------------\n")

  x := 10
  y := 20

  takes_u32(x)
  takes_i8(y)
  // takes_u32(y)
}