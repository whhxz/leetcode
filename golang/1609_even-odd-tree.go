package main

import (
	"container/list"
)

type TreeNode struct {
	Val   int
	Left  *TreeNode
	Right *TreeNode
}

func isEvenOddTree(root *TreeNode) bool {
	nodes := list.New()
	nodes.PushBack(root)
	level := 0
	for nodes.Len() > 0 {
		tmp := list.New()
		for node := nodes.Front(); node != nil; node = node.Next() {
			//偶
			current := node.Value.(*TreeNode)
			if level%2 == 0 {
				if current.Val%2 == 0 {
					return false
				}
				next := node.Next()
				if next != nil && next.Value.(*TreeNode).Val <= current.Val {
					return false
				}
			}
			//奇
			if level%2 == 1 {
				if current.Val%2 == 1 {
					return false
				}
				next := node.Next()
				if next != nil && next.Value.(*TreeNode).Val >= current.Val {
					return false
				}
			}
			if current.Left != nil {
				tmp.PushBack(current.Left)
			}
			if current.Right != nil {
				tmp.PushBack(current.Right)
			}
		}
		nodes = tmp
		level += 1
	}
	return true
}
func main() {
	n3 := &TreeNode{Val: 3}
	n10 := &TreeNode{Val: 10, Left: n3}
	n7 := &TreeNode{Val: 7}
	n9 := &TreeNode{Val: 9}
	n4 := &TreeNode{Val: 4, Left: n7, Right: n9}
	n1 := &TreeNode{Val: 1, Left: n10, Right: n4}
	println(isEvenOddTree(n1))
}

