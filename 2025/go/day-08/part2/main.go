package main

import (
	"bufio"
	"cmp"
	"fmt"
	"log"
	"os"
	"slices"
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
	result := solve(bufio.NewScanner(input), 1000)
	fmt.Printf("%s, took %s\n", result, time.Since(start))
}

type Box [3]int

func (b Box) DistanceTo(a Box) int {
	return (a[0]-b[0])*(a[0]-b[0]) + (a[1]-b[1])*(a[1]-b[1]) + (a[2]-b[2])*(a[2]-b[2])
}

type Connection struct {
	A, B     Box
	Distance int
}

func solve(scanner *bufio.Scanner, count int) string {
	var ans int

	var boxes []Box
	for scanner.Scan() {
		coords := strings.Split(scanner.Text(), ",")

		x, _ := strconv.Atoi(coords[0])
		y, _ := strconv.Atoi(coords[1])
		z, _ := strconv.Atoi(coords[2])

		boxes = append(boxes, Box{x, y, z})
	}

	var conns []Connection
	for i, a := range boxes[:len(boxes)-1] {
		for _, b := range boxes[i+1:] {
			conns = append(conns, Connection{a, b, a.DistanceTo(b)})
		}
	}
	slices.SortFunc(conns, func(a, b Connection) int { return cmp.Compare(a.Distance, b.Distance) })

	graph := make(map[Box][]Box)

	for _, conn := range conns {
		graph[conn.A] = append(graph[conn.A], conn.B)
		graph[conn.B] = append(graph[conn.B], conn.A)

		if bfs(graph, boxes[0]) == len(boxes) {
			ans = conn.A[0] * conn.B[0]
			break
		}
	}

	return strconv.Itoa(ans)
}

func bfs(graph map[Box][]Box, start Box) int {
	var size int
	visited := make(map[Box]struct{})

	queue := []Box{start}
	for len(queue) > 0 {
		curr := queue[0]
		queue = queue[1:]

		for _, next := range graph[curr] {
			if _, ok := visited[next]; !ok {
				visited[next] = struct{}{}
				queue = append(queue, next)
				size++
			}
		}
	}

	return size
}
