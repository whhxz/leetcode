package main

func minIncrements(n int, cost []int) int {
	res := 0
	for i := len(cost) - 1; i > 1; i -= 2 {
		t := cost[i] - cost[i-1]
		if t < 0 {
			res -= t
		} else {
			res += t
		}
		t = cost[i]
		if cost[i-1] > cost[i] {
			t = cost[i-1]
		}
		cost[(i>>1)-1] += t
	}
	return res
}

func main() {
	res := minIncrements(15, []int{764, 1460, 2664, 764, 2725, 4556, 5305, 8829, 5064, 5929, 7660, 6321, 4830, 7055, 3761})
	println(res)
}

