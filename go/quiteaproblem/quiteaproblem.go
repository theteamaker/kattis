package main

import (
	"bufio"
	"fmt"
	"os"
	"regexp"
)

func main() {
	scanner := bufio.NewScanner(os.Stdin)
	for scanner.Scan() {
		exp := regexp.MustCompile(`(?i)problem`)
		switch exp.MatchString(scanner.Text()) {
		case true:
			fmt.Println("yes")
		case false:
			fmt.Println("no")
		}
	}
}
