#!/bin/bash

set -e

if [ -z $1 ]; then
    echo "Usage: $0 <DAY>"
    exit
fi

if [ -z $AOC_SESSION ]; then
    echo "AOC_SESSION environment variable must be set."
    echo "Grab it from Developer Tools <F12>"
    exit
fi

DAY=$(printf "%02d" $1)
mkdir -p day-$DAY/part1

curl --silent https://adventofcode.com/2025/day/$1/input \
    --cookie session=$AOC_SESSION \
    -o day-$DAY/input.txt

cat >day-$DAY/go.mod <<EOF
module github.com/rafascar/advent-of-code/2025/go/day-$DAY

go 1.24.0
EOF

cat >day-$DAY/part1/main.go <<EOF
package main

import (
	"bufio"
	"fmt"
	"log"
	"os"
	"strconv"
)

func main() {
	input, err := os.Open("input.txt")
	if err != nil {
		log.Fatalf("Error reading input file: %v", err)
	}
	defer input.Close()

	result := solve(bufio.NewScanner(input))
	fmt.Println(result)
}

func solve(scanner *bufio.Scanner) string {
	var ans int
	return strconv.Itoa(ans)
}
EOF

cat >day-$DAY/part1/main_test.go <<EOF
package main

import (
	"bufio"
	"strings"
	"testing"
)

func TestSolve(t *testing.T) {
	input := strings.NewReader(``)

	if got, want := solve(bufio.NewScanner(input)), ""; got != want {
		t.Errorf("got = %v, want = %v", got, want)
	}
}
EOF

