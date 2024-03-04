package main

import "sort"

func bagOfTokensScore(tokens []int, power int) int {
	i, j := 0, len(tokens)-1
	if j < 0 {
		return 0
	}
	sort.Ints(tokens)
	score := 0
	for i <= j {
		if tokens[i] <= power {
			score++
			power -= tokens[i]
			i++
		} else if tokens[j] > tokens[i] && score >0{
			power += tokens[j] - tokens[i]
			i++
			j--
		} else {
			break
		}
	}

	return score
}
func main() {
	tokens := []int{77,55,82}
	res := bagOfTokensScore(tokens, 54)
	println(res)
}

