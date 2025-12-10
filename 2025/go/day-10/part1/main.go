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

type Graph map[uint16]map[uint16]struct{}

type Generator struct {
	Lights  uint16
	Buttons []uint16
}

func (g Generator) String() string {
	var s string
	s += fmt.Sprintf("[%b]", g.Lights)
	for _, b := range g.Buttons {
		s += fmt.Sprintf(" (%b)", b)
	}
	return s
}

func solve(scanner *bufio.Scanner) string {
	var ans int

	var generators []*Generator
	// Parse input converting lights and buttons to their binary representation.
	for scanner.Scan() {
		var generator Generator
		fields := strings.Fields(scanner.Text())

		for i, l := range strings.Trim(fields[0], "[]") {
			if l == '#' {
				generator.Lights |= 1 << i
			}
		}

		for _, f := range fields[1 : len(fields)-1] {
			digits := strings.Split(strings.Trim(f, "()"), ",")

			var b uint16
			for _, d := range digits {
				n, _ := strconv.Atoi(d)
				b |= 1 << n
			}
			generator.Buttons = append(generator.Buttons, b)
		}

		generators = append(generators, &generator)
	}

	// Build graph of all possible light arrangements for each generator given the available buttons.
	// Because the graph is being built in a BFS-way, as soon as we encounter the light arrangement we want
	// we can stop the search and that's the shortest path.
outer:
	for _, generator := range generators {
		graph := make(Graph)

		queue := []uint16{0}
		for i := 1; len(queue) > 0; i++ {
			for range len(queue) {
				curr := queue[0]
				queue = queue[1:]

				graph[curr] = make(map[uint16]struct{})
				for _, b := range generator.Buttons {
					next := curr ^ b
					if generator.Lights == next {
						ans += i
						continue outer
					}

					graph[curr][next] = struct{}{}
					if _, ok := graph[next]; !ok {
						queue = append(queue, next)
					}
				}
			}
		}
	}

	return strconv.Itoa(ans)
}
