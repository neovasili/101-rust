package modules

import (
	"fmt"
	"myapp/src/utils"
)

// Array assignment and access
func Array_asignment_and_access() {
	utils.PrintH3("Array assignment and access")

	var my_ints_array = [10]int8{}
	for index := range my_ints_array {
		my_ints_array[index] = 42
	}
	my_ints_array[5] = 0
	fmt.Printf("my_ints_array: %v\n", my_ints_array)
}
// Tuple assignment and access
func Tuple_assignment_and_access() {
	utils.PrintH3("Tuple assignment and access")

	var my_mixed_types_tuple = struct{int8; bool}{
		7,
		true,
	}
	fmt.Printf("1st index: %d\n", my_mixed_types_tuple.int8)
	fmt.Printf("2nd index: %t\n", my_mixed_types_tuple.bool)
	fmt.Println("NOTE: Golang does not have tuple types but we can use structs instead")
}