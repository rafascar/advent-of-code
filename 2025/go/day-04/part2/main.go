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

var dirs = [8][2]int{{-1, -1}, {0, -1}, {1, -1}, {1, 0}, {1, 1}, {0, 1}, {-1, 1}, {-1, 0}}

func solve(scanner *bufio.Scanner) string {
	var ans int

	var grid [][]string
	for scanner.Scan() {
		grid = append(grid, strings.Split(scanner.Text(), ""))
	}

	width, height := len(grid[0]), len(grid)
	// Keep trying to remove papers until there are no more left to remove.
	for {
		var removed int

		for x := range width {
			for y := range height {
				// Not a roll of paper.
				if grid[y][x] != "@" {
					continue
				}

				var count int
				for _, dir := range dirs {
					nx, ny := x+dir[0], y+dir[1]
					// Check if moving in this direction would take us out of bounds.
					if nx < 0 || ny < 0 || nx > (width-1) || ny > (height-1) {
						continue
					}
					if grid[ny][nx] == "@" {
						count++
						if count == 4 {
							break
						}
					}
				}

				if count < 4 {
					// Remove paper.
					grid[y][x] = "."
					removed++
				}
			}
		}

		ans += removed
		if removed == 0 {
			break
		}
	}

	return strconv.Itoa(ans)
}
