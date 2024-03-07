package main

type ListNode struct {
	Val  int
	Next *ListNode
}

func removeNthFromEnd(head *ListNode, n int) *ListNode {
	l := 0
	for node := head; node != nil; node = node.Next {
		l += 1
	}
	if n == l {
		return head.Next
	}
	i := 1
	for node := head; node != nil; node = node.Next {
		if i+n == l {
			node.Next = node.Next.Next
			break
		}
		i++
	}
	return head
}

