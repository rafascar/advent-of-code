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
	for line := range strings.SplitSeq(input, "\n") {
		d := line[0]
		n, err := strconv.Atoi(line[1:])
		if err != nil {
			panic(err)
		}

		switch d {
		case 'R':
			acc += n
		case 'L':
			acc -= n
		}

		for acc > 99 {
			acc -= 100
		}
		for acc < 0 {
			acc += 100
		}

		if acc == 0 {
			ans += 1
		}
	}

	return strconv.Itoa(ans)
}
