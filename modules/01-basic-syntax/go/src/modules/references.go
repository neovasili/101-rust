package modules

import (
	"fmt"
	"myapp/src/utils"
)

// References
func References() {
	utils.PrintH3("References")

	var x int32 = 10;
	var ref_x *int32 = &x;
	*ref_x = 20;
	fmt.Printf("x: %d\n", x);
}
