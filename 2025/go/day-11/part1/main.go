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

	graph := make(map[string][]string)
	for scanner.Scan() {
		line := scanner.Text()
		graph[line[:3]] = strings.Split(line[5:], " ")
	}

	ans = dfs("you", graph)

	return strconv.Itoa(ans)
}

func dfs(root string, graph map[string][]string) int {
	if root == "out" {
		return 1
	}

	var acc int
	for _, node := range graph[root] {
		acc += dfs(node, graph)
	}

	return acc
}
