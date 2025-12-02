package main

import (
	_ "embed"
	"fmt"
	"log"
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
	var res int

	for r := range strings.SplitSeq(input, ",") {
		ids := strings.Split(r, "-")

		lo, err := strconv.Atoi(ids[0])
		if err != nil {
			log.Fatal(err)
		}

		hi, err := strconv.Atoi(ids[1])
		if err != nil {
			log.Fatal(err)
		}

		for n := lo; n <= hi; n++ {
			s := strconv.Itoa(n)

			if len(s)%2 != 0 {
				continue
			}

			if s[:len(s)/2] == s[len(s)/2:] {
				res += n
			}
		}

	}
	return strconv.Itoa(res)
}
