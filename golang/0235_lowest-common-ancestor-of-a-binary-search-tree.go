package main

import (
	"container/list"
)

type TreeNode struct {
	Val   int
	Left  *TreeNode
	Right *TreeNode
}

func lowestCommonAncestor(root, p, q *TreeNode) *TreeNode {
	var dfs func(node *TreeNode, val *TreeNode, lists *list.List) bool
	dfs = func(node *TreeNode, val *TreeNode, lists *list.List) bool {
		if node == nil {
			return false
		}
		if node.Val == val.Val {
			return true
		}
		if dfs(node.Left, val, lists) {
			lists.PushFront(node.Left)
			return true
		}
		if dfs(node.Right, val, lists) {
			lists.PushFront(node.Right)
			return true
		}
		return false
	}
	pList := list.New()
	qList := list.New()

	dfs(root, p, pList)
	pList.PushFront(root)

	dfs(root, q, qList)
	qList.PushFront(root)
	pn := pList.Front()
	qn := qList.Front()

	//printList(pList)
	//printList(qList)
	for {
		if pn.Next() == nil {
			return pn.Value.(*TreeNode)
		}
		if qn.Next() == nil {
			return qn.Value.(*TreeNode)
		}
		if pn.Next().Value.(*TreeNode).Val != qn.Next().Value.(*TreeNode).Val {
			return pn.Value.(*TreeNode)
		} else {
			pn = pn.Next()
			qn = qn.Next()
		}
	}
}
func printList(lists *list.List) {
	for n := lists.Front(); n != nil; n = n.Next() {
		print(n.Value.(*TreeNode).Val)
		print(" ")
	}
	println()
}

func main() {
	n3 := &TreeNode{Val: 3}
	n5 := &TreeNode{Val: 5}
	n4 := &TreeNode{Val: 4, Left: n3, Right: n5}
	n0 := &TreeNode{Val: 0}
	n2 := &TreeNode{Val: 2, Left: n0, Right: n4}
	n7 := &TreeNode{Val: 7}
	n9 := &TreeNode{Val: 9}
	n8 := &TreeNode{Val: 8, Left: n7, Right: n9}
	n6 := &TreeNode{Val: 6, Left: n2, Right: n8}

	ancestor := lowestCommonAncestor(n6, n3, n5)
	println(ancestor.Val)
}
