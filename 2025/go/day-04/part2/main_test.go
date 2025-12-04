package main

import (
	"bufio"
	"strings"
	"testing"
)

func TestSolve(t *testing.T) {
	input := strings.NewReader(`..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@.`)

	if got, want := solve(bufio.NewScanner(input)), "43"; got != want {
		t.Errorf("got = %v, want = %v", got, want)
	}
}
