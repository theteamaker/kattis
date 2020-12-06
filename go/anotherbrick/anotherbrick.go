package main

import (
	"os"
	"bufio"
	"strconv"
	"strings"
	"fmt"
)

func main() {
	scanner := bufio.NewScanner(os.Stdin)
	var lines []string

	for scanner.Scan() {
		lines = append(lines, scanner.Text())
	}

	var measure = num(strings.Split(lines[0], " "))
	var bricks = num(strings.Split(lines[1], " "))

	h, w := measure[0], measure[1]
	count := 0

	for i := 0; i < h; i++ {
		for count < w {
			count += bricks[0]
			copy(bricks[0:], bricks[0+1:])
			bricks[len(bricks)-1] = 0
			bricks = bricks[:len(bricks)-1]
		}
		
		switch count {
		case w:
			count = 0
			continue
		default:
			fmt.Println("NO")
			os.Exit(0)
		}
	}

	fmt.Println("YES")
}

func num(x []string) []int {
	var nums []int
	for _, i := range x {
		n, err := strconv.Atoi(i)
		if err != nil {
			panic(err)
		}
		nums = append(nums, n)
	}
	return nums
}