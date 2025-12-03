package main

import (
	"bufio"
	"strings"
	"testing"
)

func TestSolve(t *testing.T) {
	input := strings.NewReader(`987654321111111
811111111111119
234234234234278
818181911112111`)

	if got, want := solve(bufio.NewScanner(input)), "3121910778619"; got != want {
		t.Errorf("got = %v, want = %v", got, want)
	}
}
