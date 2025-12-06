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

	var matrix [][]string
	for scanner.Scan() {
		matrix = append(matrix, strings.Fields(scanner.Text()))
	}

	rows, cols := len(matrix), len(matrix[0])
	for c := range cols {
		op := matrix[rows-1][c]

		var acc int
		if op == "*" {
			acc = 1
		}

		for r := range rows - 1 {
			n, _ := strconv.Atoi(matrix[r][c])
			switch op {
			case "+":
				acc += n
			case "*":
				acc *= n
			}
		}

		ans += acc
	}

	return strconv.Itoa(ans)
}
