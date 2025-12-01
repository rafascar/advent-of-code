package main

import (
	_ "embed"
	"fmt"
	"os"
	"strconv"
	"strings"
)

func main() {
	input, err := os.ReadFile("input.txt")
	if err != nil {
		panic(err)
	}

	inputStr := string(input)
	inputStr = strings.TrimSuffix(inputStr, "\n")

	result := process(inputStr)
	fmt.Println(result)
}

func process(input string) string {
	var ans int

	acc := 50
	var sign int
	for line := range strings.SplitSeq(input, "\n") {
		d := line[0]
		n, err := strconv.Atoi(line[1:])
		if err != nil {
			panic(err)
		}

		switch d {
		case 'R':
			sign = 1
		case 'L':
			sign = -1
		}

		for range n {
			acc += sign
			if acc > 99 {
				acc = 0
			} else if acc < 0 {
				acc = 99
			}

			if acc == 0 {
				ans += 1
			}
		}
	}

	return strconv.Itoa(ans)
}
