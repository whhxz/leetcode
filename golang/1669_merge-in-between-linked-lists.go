package main

type ListNode struct {
	Val  int
	Next *ListNode
}

func mergeInBetween(list1 *ListNode, a int, b int, list2 *ListNode) *ListNode {
	head := &ListNode{
		Val:  0,
		Next: list1,
	}
	node := head
	i := 0

	first := head
	second := head
	for node != nil {
		if i == a {
			first = node
		}
		if i == b {
			second = node.Next.Next
			break
		}
		i += 1
		node = node.Next
	}
	first.Next = list2
	node = list2
	for node != nil {
		if node.Next == nil {
			node.Next = second
			break
		}
		node = node.Next
	}
	return head.Next
}

func buildNode(start, end int) *ListNode {
	head := &ListNode{
		Val: start,
	}
	node := head
	for i := start + 1; i <= end; i++ {
		node.Next = &ListNode{
			Val: i,
		}
		node = node.Next
	}
	return head
}
func compare(nums []int, node *ListNode) {
	i := 0
	for node != nil {
		if nums[i] != node.Val {
			panic(nums[i])
		}
		i+=1
		node = node.Next
	}
}
func main() {
	list1 := buildNode(0, 6)
	list2 := buildNode(1000, 1004)
	res := mergeInBetween(list1, 2, 5, list2)
	compare([]int{0,1,1000,1001,1002,1003,1004,6}, res)

	list1 = buildNode(0, 6)
	list2 = buildNode(1000, 1004)
	res = mergeInBetween(list1, 2, 6, list2)
	compare([]int{0,1,1000,1001,1002,1003,1004}, res)

	list1 = buildNode(0, 6)
	list2 = buildNode(1000, 1004)
	res = mergeInBetween(list1, 0, 5, list2)
	compare([]int{1000,1001,1002,1003,1004,6}, res)
	list1 = buildNode(0, 6)
	list2 = buildNode(1000, 1004)
	res = mergeInBetween(list1, 0, 6, list2)
	compare([]int{1000,1001,1002,1003,1004}, res)
}

