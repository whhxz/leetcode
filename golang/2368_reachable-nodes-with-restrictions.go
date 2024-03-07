package main

func reachableNodes(n int, edges [][]int, restricted []int) int {
	graph := make(map[int][]int)
	for _, d := range edges {
		a, b := d[0], d[1]
		if _, ok := graph[a]; !ok {
			graph[a] = []int{}
		}
		if _, ok := graph[b]; !ok {
			graph[b] = []int{}
		}
		graph[a] = append(graph[a], b)
		graph[b] = append(graph[b], a)
	}
	cache := make([]bool, n)
	for _, d := range restricted {
		cache[d] = true
	}
	var dfs func(d int)
	res := 0
	dfs = func(d int) {
		if cache[d]{
			return
		}
		res++
		cache[d] = true
		if ts, ok := graph[d]; ok {
			for _, t := range ts {
				dfs(t)
			}
		}
	}
	dfs(0)
	return res
}
func main() {
	edges := [][]int{{0,1},{1,2},{3,1},{4,0},{0,5},{5,6}}
	restricted:=[]int{4,5}
	res := reachableNodes(7, edges, restricted)
	println(res)
}
