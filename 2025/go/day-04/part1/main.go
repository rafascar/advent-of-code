package main

import (
	"bufio"
	"fmt"
	"log"
	"os"
	"strconv"
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

var dirs = [8][2]int{{-1, -1}, {0, -1}, {1, -1}, {1, 0}, {1, 1}, {0, 1}, {-1, 1}, {-1, 0}}

func solve(scanner *bufio.Scanner) string {
	var ans int

	var grid []string
	for scanner.Scan() {
		grid = append(grid, scanner.Text())
	}

	width, height := len(grid[0]), len(grid)
	for i := range width {
		for j := range height {
			// Not a roll of paper.
			if grid[j][i] != '@' {
				continue
			}

			var count int
			for _, dir := range dirs {
				// Check if moving in this direction would take us out of bounds.
				if (dir[0] < 0 && i == 0) || (dir[0] > 0 && i == (width-1)) || (dir[1] < 0 && j == 0) || (dir[1] > 0 && j == (height-1)) {
					continue
				}
				if grid[j+dir[1]][i+dir[0]] == '@' {
					count++
					if count == 4 {
						break
					}
				}
			}

			if count < 4 {
				ans++
			}
		}
	}

	return strconv.Itoa(ans)
}
