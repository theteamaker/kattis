package main

import (
	"bufio"
	"fmt"
	"os"
	"strconv"
	"strings"
)

type definition struct {
	name  string
	value int
}

func main() {
	scanner := bufio.NewScanner(os.Stdin)
	defined := make(map[string]int)

	for scanner.Scan() {
		command := strings.Split(scanner.Text(), " ")

		switch command[0] {
		case "define":
			num, _ := strconv.Atoi(command[1])
			defined[command[2]] = num

		case "eval":
			_, ok1 := defined[command[1]]
			_, ok2 := defined[command[3]]

			if !(ok1) || !(ok2) {
				fmt.Println("undefined")
				break
			}

			switch command[2] {
			case "<":
				fmt.Println(defined[command[1]] < defined[command[3]])
			case ">":
				fmt.Println(defined[command[1]] > defined[command[3]])
			case "=":
				fmt.Println(defined[command[1]] == defined[command[3]])
			}
		}
	}
}
