type TreeNode struct {
	Val   int
	Left  *TreeNode
	Right *TreeNode
}

func buildTree(inorder []int, postorder []int) *TreeNode {
	inorderMap := map[int]int{}
	for i, v := range inorder {
		inorderMap[v] = i
	}
	postorderRight := len(postorder) - 1
	var build func(left, right int) *TreeNode
	build = func(left, right int) *TreeNode {
		if left > right {
			return nil
		}
		val := postorder[postorderRight]
		postorderRight--
		node := &TreeNode{Val: val}
		nodeIndex := inorderMap[val]
		node.Right = build(nodeIndex+1, right)
		node.Left = build(left, nodeIndex-1)
		return node
	}
	return build(0, len(inorder)-1)
}
