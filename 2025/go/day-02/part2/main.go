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

	for seq := range strings.SplitSeq(input, ",") {
		bounds := strings.Split(seq, "-")

		lo, err := strconv.Atoi(bounds[0])
		if err != nil {
			log.Fatal(err)
		}

		hi, err := strconv.Atoi(bounds[1])
		if err != nil {
			log.Fatal(err)
		}

		for n := lo; n <= hi; n++ {
			s := strconv.Itoa(n)

		outer:
			// 'r'ight pointer that points to the end of the pattern (non-inclusive).
			// Loop until we reach half of the string as the minimum pattern is repeating twice.
			for r := 1; r <= len(s)/2; r++ {
				// String length needs to be a multiple of pattern length.
				if len(s)%r != 0 {
					continue
				}

				// Check if pattern repeats through the whole string.
				pat := s[:r]
				for i := 1; i < len(s)/(r); i++ {
					if pat != s[i*r:(i+1)*r] {
						continue outer
					}
				}

				// If we got here, we have a pattern!
				res += n
				break
			}
		}
	}
	return strconv.Itoa(res)
}
