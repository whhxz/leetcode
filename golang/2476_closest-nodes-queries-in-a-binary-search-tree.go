package main

import (
	"fmt"
	"sort"
)

// TODO 一个可以直接更快树搜索
type TreeNode struct {
	Val   int
	Left  *TreeNode
	Right *TreeNode
}

func closestNodes(root *TreeNode, queries []int) [][]int {
	var datas []int
	var toArray func(node *TreeNode)
	toArray = func(node *TreeNode) {
		if node == nil {
			return
		}
		toArray(node.Left)
		datas = append(datas, node.Val)
		toArray(node.Right)
	}
	toArray(root)
	fmt.Printf("%v\n", datas)

	var find func(i, left, right int) int
	find = func(val, left, right int) int {
		if left > right {
			return left
		}
		center := (left + right) >> 1
		if datas[center] == val {
			return center
		}
		if val > datas[center] {
			return find(val, center+1, right)
		} else {
			return find(val, left, center-1)
		}
	}
	ql := len(datas)
	res := make([][]int, len(queries))
	for i, q := range queries {
		fi := sort.SearchInts(datas, q)
		//fi := find(q, 0, ql-1)
		//fmt.Printf("%d,%d\n", t, fi)
		if fi == ql {
			res[i] = []int{datas[ql-1], -1}
		} else if datas[fi] == q {
			res[i] = []int{q, q}
		} else if fi == 0 {
			res[i] = []int{-1, datas[0]}
		} else {
			res[i] = []int{datas[fi-1], datas[fi]}
		}
	}
	//fmt.Printf("%v\n", res)
	return res
}
func main() {
	t1 := &TreeNode{Val: 1}
	t4 := &TreeNode{Val: 4}
	t2 := &TreeNode{Val: 2, Left: t1, Right: t4}

	t14 := &TreeNode{Val: 14}
	t15 := &TreeNode{Val: 15, Left: t14}

	t9 := &TreeNode{Val: 9}
	t13 := &TreeNode{Val: 13, Left: t9, Right: t15}

	t6 := &TreeNode{Val: 6, Left: t2, Right: t13}

	closestNodes(t6, []int{2, 5, 16})
}
