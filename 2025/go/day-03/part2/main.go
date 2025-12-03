package main

import (
	"bufio"
	"fmt"
	"log"
	"math"
	"os"
	"strconv"
	"time"
)

func main() {
	start := time.Now()

	input, err := os.Open("input.txt")
	if err != nil {
		log.Fatalf("Error reading input file: %v", err)
	}
	defer input.Close()

	result := solve(bufio.NewScanner(input))
	fmt.Printf("%s, took %s\n", result, time.Since(start))
}

func solve(scanner *bufio.Scanner) string {
	var ans int

	for scanner.Scan() {
		line := scanner.Text()

		batteries := make([]int, 0, len(line))
		for _, b := range line {
			batteries = append(batteries, int(b-'0'))
		}

		var bank int
		var p int
		for i := range 12 {
			b := batteries[p]
			for j := p + 1; j < len(line)-11+i; j++ {
				if batteries[j] > b {
					b, p = batteries[j], j
				}
			}
			bank += b * int(math.Pow10(11-i))
			p++
		}

		ans += bank
	}

	return strconv.Itoa(ans)
}
