package main

import (
	"bufio"
	"fmt"
	"log"
	"os"
	"strconv"
	"strings"
	"time"
)

type Range struct {
	Start int
	End   int
}

func main() {
	input, err := os.Open("input.txt")
	if err != nil {
		log.Fatalf("Error reading input file: %v", err)
	}
	defer input.Close()

	start := time.Now()
	result := solve(bufio.NewScanner(input))
	fmt.Printf("%s, took %s\n", result, time.Since(start))
}

func solve(scanner *bufio.Scanner) string {
	var ans int

	var ranges []Range
	for scanner.Scan() {
		r := strings.Split(scanner.Text(), "-")
		// Blank line marks the end of the ranges.
		if len(r) < 2 {
			break
		}

		start, _ := strconv.Atoi(r[0])
		end, _ := strconv.Atoi(r[1])
		ranges = append(ranges, Range{Start: start, End: end})

	}

	for scanner.Scan() {
		n, _ := strconv.Atoi(scanner.Text())

		for _, r := range ranges {
			if n >= r.Start && n <= r.End {
				ans++
				break
			}
		}
	}

	return strconv.Itoa(ans)
}
