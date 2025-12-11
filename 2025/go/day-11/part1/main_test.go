package main

import (
	"bufio"
	"strings"
	"testing"
)

func TestSolve(t *testing.T) {
	input := strings.NewReader(`aaa: you hhh
you: bbb ccc
bbb: ddd eee
ccc: ddd eee fff
ddd: ggg
eee: out
fff: out
ggg: out
hhh: ccc fff iii
iii: out`)

	if got, want := solve(bufio.NewScanner(input)), "5"; got != want {
		t.Errorf("got = %v, want = %v", got, want)
	}
}
