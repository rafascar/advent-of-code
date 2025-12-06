package main

import (
	"bufio"
	"strings"
	"testing"
)

func TestSolve(t *testing.T) {
	input := strings.NewReader(`123 328  51 64 
 45 64  387 23 
  6 98  215 314
*   +   *   +  `)

	if got, want := solve(bufio.NewScanner(input)), "3263827"; got != want {
		t.Errorf("got = %v, want = %v", got, want)
	}
}

// Transposed input:
// 1  *
// 24
// 356
//
// 369+
// 248
// 8
//
//  32*
// 581
// 175
//
// 623+
// 431
//   4
