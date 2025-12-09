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

	var rows, cols int
	var tiles [][2]int
	for scanner.Scan() {
		coords := strings.Split(scanner.Text(), ",")

		c, _ := strconv.Atoi(coords[0])
		r, _ := strconv.Atoi(coords[1])
		rows, cols = max(rows, r+1), max(cols, c+1)

		tiles = append(tiles, [2]int{r, c})
	}
	// Append first tile to the end to make it easier to close the loop.
	tiles = append(tiles, tiles[0])

	grid := make([][]string, rows)
	for r := range rows {
		grid[r] = make([]string, cols)
		for c := range cols {
			grid[r][c] = "."
		}
		// fmt.Println(grid[r])
	}

	// Draw tile boundaries.
	curr := tiles[0]
	for _, tile := range tiles[1:] {
		if curr[0] < tile[0] {
			for r := curr[0]; r <= tile[0]; r++ {
				grid[r][curr[1]] = "#"
			}
		} else if curr[0] > tile[0] {
			for r := tile[0]; r <= curr[0]; r++ {
				grid[r][curr[1]] = "#"
			}
		} else if curr[1] < tile[1] {
			for c := curr[1]; c <= tile[1]; c++ {
				grid[curr[0]][c] = "#"
			}
		} else {
			for c := tile[1]; c <= curr[1]; c++ {
				grid[curr[0]][c] = "#"
			}
		}
		curr = tile
	}

	// fmt.Println()
	// for r := range rows {
	// 	fmt.Println(grid[r])
	// }

	// Raycast from left to right to fill the loop.
	for r := range rows {
		var crossed bool
		var inside bool

		for c := range cols {
			switch grid[r][c] {
			case "#":
				crossed = true
			case ".":
				if crossed {
					crossed = false
					inside = !inside
				}
				if inside {
					grid[r][c] = "X"
				}
			}
		}
	}

	// fmt.Println()
	// for r := range rows {
	// 	fmt.Println(grid[r])
	// }

	for i, a := range tiles[:len(tiles)-1] {
	outer:
		for _, b := range tiles[i+1:] {
			for r := min(a[0], b[0]); r < max(a[0], b[0]); r++ {
				for c := min(a[1], b[1]); c < max(a[1], b[1]); c++ {
					if grid[r][c] == "." {
						continue outer
					}
				}
			}

			area := (a[0] - b[0] + 1) * (a[1] - b[1] + 1)
			if area > ans {
				ans = area
			}
		}
	}

	return strconv.Itoa(ans)
}
