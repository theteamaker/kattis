package main

import (
	"bufio"
	"fmt"
	"os"
	"strconv"
	"strings"
	"unicode"
)

func main() {
	scanner := bufio.NewScanner(os.Stdin)
	var lines []string

	for scanner.Scan() {
		lines = append(lines, scanner.Text())
	}

	if (lines[0] == lines[1]) || ((lines[0]) == uppercheck(lines[1])) {
		fmt.Println("Yes")
		os.Exit(0)
	}

	for i := 0; i <= 9; i++ {
		switch lines[0] {
		case (lines[1] + strconv.Itoa(i)):
			fmt.Println("Yes")
			os.Exit(0)
		case (strconv.Itoa(i) + lines[1]):
			fmt.Println("Yes")
			os.Exit(0)
			break
		default:
			continue
		}
	}

	fmt.Println("No") // if all checks fail
}

func uppercheck(str string) string {
	var letters = strings.Split(str, "")
	var reversed_letters []string
	for _, x := range letters {
		for _, y := range x {
			switch unicode.IsUpper(y) {
			case true:
				reversed_letters = append(reversed_letters, strings.ToLower(x))
			case false:
				reversed_letters = append(reversed_letters, strings.ToUpper(x))
			}
		}
	}
	return strings.Join(reversed_letters, "")
}
