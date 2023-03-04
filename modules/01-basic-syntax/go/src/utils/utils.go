package utils

import (
	"fmt"
	"github.com/ttacon/chalk"
)

func PrintH1(title string) {
	fmt.Printf("\n%s\n", chalk.Blue.Color(title))
	PrintBr()
}

func PrintH2(title string) {
	fmt.Printf("\n%s\n", chalk.Magenta.Color(title))
}

func PrintH3(title string) {
	fmt.Printf("\n\n%s\n", chalk.Yellow.Color(title))
}

func PrintBr() {
	fmt.Println("------------")
}
