package modules

import (
	"fmt"
	"myapp/src/utils"
)

// Array slices
func Array_slices() {
  utils.PrintH3("Array slices");

  my_ints_array := [6]int32{10, 20, 30, 40, 50, 60}
  fmt.Printf("my_ints_array: %v\n", my_ints_array)

  var my_ints_slice []int32 = my_ints_array[2:4]
  fmt.Printf("my_ints_slice: %v\n", my_ints_slice)
}

// String vs &str
func String_vs_str() {
  utils.PrintH3("String vs &str")

  s1 := "World"
  fmt.Printf("s1: %s\n", s1)

  s2 := "Hello "
  fmt.Printf("s2: %s\n", s2)
  s2 += s1
  fmt.Printf("s2: %s\n", s2)

  s3 := s2[6:]
  fmt.Printf("s3: %s\n", s3)
  fmt.Println("NOTE: Golang does not have different types of string")
}
