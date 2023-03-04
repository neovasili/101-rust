package main

import (
	"myapp/src/modules"
	"myapp/src/utils"
)

func main() {
	utils.PrintH1("Basic syntax")

  // Compound types
	utils.PrintH2("Compound types")
	modules.Array_asignment_and_access()
	modules.Tuple_assignment_and_access()
	utils.PrintBr()

	// References
	utils.PrintH2("References")
	modules.References()
	utils.PrintBr()

	// Slices
	utils.PrintH2("Slices")
	modules.Array_slices()
	modules.String_vs_str()
	utils.PrintBr()

	// Functions
	utils.PrintH2("Functions")
	modules.Fizzbuzz_to(10)
	modules.Test_methods_from_rectangle()
	modules.Test_overloading()
	utils.PrintBr()
}
