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

	var input []string
	for scanner.Scan() {
		input = append(input, scanner.Text())
	}

	// Transpose input matrix.
	rows, cols := len(input), len(input[0])
	trans := make([]string, cols)
	for c := range cols {
		for r := range rows {
			trans[c] += string(input[r][c])
		}
	}

	for i := 0; i < len(trans); {
		acc, size := computeBlock(trans[i:])
		ans += acc
		i += size + 1
	}

	return strconv.Itoa(ans)
}

// Compute the total for a single block and how many lines that block is made of.
func computeBlock(block []string) (total, size int) {
	cols := len(block[0])

	// In the transposed input, the first line of a block
	// always contains the operator on the last column.
	op := block[0][cols-1]
	var acc int
	if op == '*' {
		acc = 1
	}

	var i int
	for i = range block {
		line := strings.Trim(block[i][:cols-1], " ")
		// Blocks are separated by one all-whitespace line.
		if line == "" {
			break
		}

		n, _ := strconv.Atoi(line)
		switch op {
		case '+':
			acc += n
		case '*':
			acc *= n
		}
	}

	return acc, i
}
