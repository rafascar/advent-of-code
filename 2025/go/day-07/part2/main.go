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

	// Find initial beam.
	scanner.Scan()
	beams := make(map[int]int)
	beams[strings.Index(scanner.Text(), "S")] = 1

	for scanner.Scan() {
		line := scanner.Text()

		splitBeams := make(map[int]int)
		for b := range beams {
			if line[b] == '^' {
				splitBeams[b-1] += beams[b]
				splitBeams[b+1] += beams[b]
			} else {
				splitBeams[b] += beams[b]
			}
		}
		beams = splitBeams
	}

	for _, v := range beams {
		ans += v
	}

	return strconv.Itoa(ans)
}
