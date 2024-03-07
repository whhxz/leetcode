package main

func findJudge(n int, trust [][]int) int {
	if len(trust) == 0{
		if n == 1{
			return 1
		} else {
			return -1
		}
	}
	//{出数量, 入数量}
	data := make([][]int, n+1)
	for _, t := range trust {
		if len(data[t[0]]) == 0 {
			data[t[0]] = []int{0, 0}
		}
		if len(data[t[1]]) == 0 {
			data[t[1]] = []int{0, 0}
		}
		data[t[0]][0]++
		data[t[1]][1]++
	}
	for i, d := range data {
		if len(d) != 2 {
			continue
		}
		if d[0] == 0 && d[1] == n-1 {
			return i
		}
	}
	return -1
}
func main() {
	trust := [][]int{}
	d := findJudge(1, trust)
	println(d)
}
