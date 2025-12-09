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

	if got, want := solve(bufio.NewScanner(input)), "24"; got != want {
		t.Errorf("got = %v, want = %v", got, want)
	}
}

// Example input
// ..............
// .......#XXX#..
// .......X...X..
// ..#XXXX#...X..
// ..X........X..
// ..#XXXXXX#.X..
// .........X.X..
// .........#X#..
// ..............

// Example solution
// ..............
// .......#XXX#..
// .......XXXXX..
// ..OOOOOOOOXX..
// ..OOOOOOOOXX..
// ..OOOOOOOOXX..
// .........XXX..
// .........#X#..
// ..............
