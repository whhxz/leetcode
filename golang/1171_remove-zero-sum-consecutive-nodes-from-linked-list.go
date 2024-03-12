package main

type ListNode struct {
	Val  int
	Next *ListNode
}

func removeZeroSumSublists(head *ListNode) *ListNode {
	head = &ListNode{Val: 0, Next: head}
	first := head
	for first != nil {
		second := first.Next
		total := 0
		for second != nil {
			total += second.Val
			if total == 0 {
				first.Next = second.Next
			}
			second = second.Next
		}
		first = first.Next
	}
	return head.Next
}
func main() {
	n5 := &ListNode{Val: 1}
	n4 := &ListNode{Val: 3, Next: n5}
	n3 := &ListNode{Val: -3, Next: n4}
	n2 := &ListNode{Val: 2, Next: n3}
	n1 := &ListNode{Val: 1, Next: n2}
	n0 := &ListNode{Val: 0, Next: n1}
	n0 = removeZeroSumSublists(n0)
	n := n0
	for n != nil{
		print(n.Val)
		print("\t")
		n = n.Next
	}
}
