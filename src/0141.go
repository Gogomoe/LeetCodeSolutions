package main

/**
 * Definition for singly-linked list.
 * type ListNode struct {
 *     Val int
 *     Next *ListNode
 * }
 */
func hasCycle(head *ListNode) bool {
	p1 := head
	p2 := head
	for {
		if p2 == nil || p2.Next == nil {
			return false
		}
		p2 = p2.Next.Next
		p1 = p1.Next
		if p1 == p2 {
			break
		}
	}
	return true
}
