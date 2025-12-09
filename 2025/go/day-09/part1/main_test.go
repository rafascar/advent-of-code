package main

import (
	"bufio"
	"strings"
	"testing"
)

func TestSolve(t *testing.T) {
	input := strings.NewReader(`7,1
11,1
11,7
9,7
9,5
2,5
2,3
7,3`)

	if got, want := solve(bufio.NewScanner(input)), "50"; got != want {
		t.Errorf("got = %v, want = %v", got, want)
	}
}

// Example input
// ..............
// .......#...#..
// ..............
// ..#....#......
// ..............
// ..#......#....
// ..............
// .........#.#..
// ..............

// Example solution
// ..............
// ..OOOOOOOOOO..
// ..OOOOOOOOOO..
// ..OOOOOOOOOO..
// ..OOOOOOOOOO..
// ..OOOOOOOOOO..
// ..............
// .........#.#..
// ..............
