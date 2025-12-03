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

	for scanner.Scan() {
		line := scanner.Text()

		nums := make([]int, 0, len(line))
		for _, c := range line {
			nums = append(nums, int(c-'0'))
		}

		var bat int
		// s marks where to start searching for the next digit from.
		var s int
		for i := range 12 {
			d := nums[s]
			for j := s + 1; j < len(line)-11+i; j++ {
				// Found a better candidate: update the digit and the next search point.
				if nums[j] > d {
					d = nums[j]
					s = j
				}
			}
			bat += d * int(math.Pow10(11-i))
			s++
		}
		ans += bat
	}

	return strconv.Itoa(ans)
}
