package main

import "sort"

type TreeNode struct {
	Val   int
	Left  *TreeNode
	Right *TreeNode
}

func kthLargestLevelSum(root *TreeNode, k int) int64 {
	var dfs func(node *TreeNode, deep int)
	container := make([]int64, 1)
	dfs = func(node *TreeNode, deep int) {
		if node == nil {
			return
		}
		if len(container) == deep {
			container = append(container, 0)
		}
		container[deep] += int64(node.Val)
		dfs(node.Left, deep+1)
		dfs(node.Right, deep+1)
	}
	dfs(root, 0)
	if len(container) <= k-1 {
		return -1
	}
	sort.Slice(container, func(i, j int) bool { return container[i] > container[j] })
	return container[k-1]
}
