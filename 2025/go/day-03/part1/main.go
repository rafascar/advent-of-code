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

func solve(scanner *bufio.Scanner) string {
	var ans int

	for scanner.Scan() {
		line := scanner.Text()

		nums := make([]int, 0, len(line))
		for _, c := range line {
			nums = append(nums, int(c-'0'))
		}

		bat, _ := strconv.Atoi(line[0:2])
		l, r := nums[0], nums[1]
		for _, n := range nums[1:] {
			if n > r {
				r = n
				bat = l*10 + r
			}

			if n > l {
				l = n
				r = -1
			}
		}

		ans += bat
	}

	return strconv.Itoa(ans)
}
