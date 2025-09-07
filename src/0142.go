package main

/**
 * Definition for singly-linked list.
 * type ListNode struct {
 *     Val int
 *     Next *ListNode
 * }
 */
func detectCycle(head *ListNode) *ListNode {
	p1 := head
	p2 := head
	for {
		if p2 == nil || p2.Next == nil {
			return nil
		}
		p2 = p2.Next.Next
		p1 = p1.Next
		if p1 == p2 {
			break
		}
	}
	p3 := head
	for {
		if p1 == p3 {
			break
		}
		p1 = p1.Next
		p3 = p3.Next
	}
	return p3
}
