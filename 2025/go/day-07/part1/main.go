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
	beams := make(map[int]struct{})
	beams[strings.Index(scanner.Text(), "S")] = struct{}{}

	for scanner.Scan() {
		line := scanner.Text()

		splitBeams := make(map[int]struct{})
		for b := range beams {
			if line[b] == '^' {
				splitBeams[b-1] = struct{}{}
				splitBeams[b+1] = struct{}{}
				ans++
			} else {
				splitBeams[b] = struct{}{}
			}
		}
		beams = splitBeams
	}

	return strconv.Itoa(ans)
}
