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

	var tiles [][2]int
	for scanner.Scan() {
		coords := strings.Split(scanner.Text(), ",")
		c, _ := strconv.Atoi(coords[0])
		r, _ := strconv.Atoi(coords[1])
		tiles = append(tiles, [2]int{r, c})
	}

	for i, a := range tiles[:len(tiles)-1] {
		for _, b := range tiles[i+1:] {
			area := (a[0] - b[0] + 1) * (a[1] - b[1] + 1)
			if area > ans {
				ans = area
			}
		}
	}

	return strconv.Itoa(ans)
}
