package main

type ListNode struct {
	Val  int
	Next *ListNode
}

func hasCycle(head *ListNode) bool {
	fast, slow := head, head
	for fast != nil && fast.Next != nil {
		if fast.Next == slow {
			return true
		}
		fast = fast.Next.Next
		slow = slow.Next
	}
	return false
}

