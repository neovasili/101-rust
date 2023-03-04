package main

import (
	"fmt"
)

// Scopes and shadowing
func main() {
  fmt.Println("------------------------\n")

  a := 10
  fmt.Printf("before: %v\n", a)

  {
    a := "hello"
    fmt.Printf("inner scope: %v\n", a)

    a := true
    fmt.Printf("shadowed in inner scope: %v\n", a)
  }

  fmt.Printf("after: %v\n", a)
}
