package main

import "strings"

func maximumOddBinaryNumber(s string) string {
	num := 0
	for _, c := range s {
		if c == '1' {
			num += 1
		}
	}
	if num == 0 || num == len(s) {
		return s
	}
	if num == 1 {
		return strings.Repeat("0", len(s)-1) + "1"
	}
	return strings.Repeat("1", num-1) + strings.Repeat("0", len(s)-num) + "1"
}
func main() {
	s := maximumOddBinaryNumber("0101")
	println(s)
}

