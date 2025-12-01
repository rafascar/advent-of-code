package main

import "testing"

func TestProcess(t *testing.T) {
	input := `L68
L30
R48
L5
R60
L55
L1
L99
R14
L82`

	if got, want := process(input), "3"; got != want {
		t.Errorf("got = %v, want = %v", got, want)
	}
}
