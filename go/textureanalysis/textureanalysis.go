package main

import (
	"bufio"
	"fmt"
	"os"
	"strings"
)

func main() {
	scanner := bufio.NewScanner(os.Stdin)
	count := 0

	for scanner.Scan() {
		count++
		if scanner.Text() == "END" {
			break
		}

		lines := strings.Split(scanner.Text(), "")

		var asterisks []int
		for c, i := range lines {
			if i == "*" {
				asterisks = append(asterisks, c)
			}
		}

		if len(asterisks) == 1 {
			fmt.Println(count, "EVEN")
			continue
		}

		expected_distance := asterisks[1] - asterisks[0]
		good := true
		for i := 1; i <= len(asterisks)-1; i++ {
			switch asterisks[i] - asterisks[i-1] {
			case expected_distance:
				continue
			default:
				good = false
				break
			}
			break
		}

		switch good {
		case true:
			fmt.Println(count, "EVEN")
		case false:
			fmt.Println(count, "NOT EVEN")
		}
	}
}
