package main

import (
	"bufio"
	"fmt"
	"os"
	"strconv"
	"strings"
)

func main() {
	values := map[string]string{
		"A": "11 11",
		"K": "4 4",
		"Q": "3 3",
		"J": "20 2",
		"T": "10 10",
		"9": "14 0",
		"8": "0 0",
		"7": "0 0",
	}

	scanner := bufio.NewScanner(os.Stdin)
	var cards []string
	value := 0

	scanner.Scan()
	line_one := strings.Split(scanner.Text(), " ")

	dominant_suit := line_one[1]

	for scanner.Scan() {
		cards = append(cards, scanner.Text())
	}

	for _, i := range cards {
		line := strings.Split(i, "")
		card, suit := line[0], line[1]

		vals := strings.Split(values[card], " ")

		switch suit {
		case dominant_suit:
			i, _ := strconv.Atoi(vals[0])
			value += i
		default:
			i, _ := strconv.Atoi(vals[1])
			value += i
		}
	}

	fmt.Println(value)
}
