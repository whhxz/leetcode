package main

import "container/list"

type TreeNode struct {
	Val   int
	Left  *TreeNode
	Right *TreeNode
}

func findBottomLeftValue(root *TreeNode) int {
	nodes := list.New()
	nodes.PushBack(root)
	var res *TreeNode
	for nodes.Len() > 0 {
		tmp := list.New()
		res = nodes.Front().Value.(*TreeNode)
		for t := nodes.Front(); t != nil; t = t.Next() {
			node := t.Value.(*TreeNode)
			if node.Left != nil {
				tmp.PushBack(node.Left)
			}
			if node.Right != nil {
				tmp.PushBack(node.Right)
			}
		}
		nodes = tmp
	}
	return res.Val
}

