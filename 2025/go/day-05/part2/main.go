package main

import (
	"bufio"
	"cmp"
	"fmt"
	"log"
	"os"
	"slices"
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
	slices.SortFunc(ranges, func(a, b Range) int { return cmp.Compare(a.Start, b.Start) })

	mergedRanges := ranges[:1]
	for _, r := range ranges[1:] {
		mr := &mergedRanges[len(mergedRanges)-1]
		if r.Start <= mr.End {
			mr.End = max(r.End, mr.End)
		} else {
			mergedRanges = append(mergedRanges, r)
		}
	}

	for _, r := range mergedRanges {
		ans += r.End - r.Start + 1
	}

	return strconv.Itoa(ans)
}
