package main

import (
	"bufio"
	"strings"
	"testing"
)

func TestSolve(t *testing.T) {
	input := strings.NewReader(`3-5
10-14
16-20
12-18

1
5
8
11
17
32`)

	if got, want := solve(bufio.NewScanner(input)), "14"; got != want {
		t.Errorf("got = %v, want = %v", got, want)
	}
}
