package main

type TreeNode struct {
	Val   int
	Left  *TreeNode
	Right *TreeNode
}

// Linked List
type ListNode struct {
	Val  int
	Next *ListNode
}

func ToList(arr []int) *ListNode {
	head := &ListNode{}
	last := head
	for _, val := range arr {
		last.Next = &ListNode{
			Val: val,
		}
		last = last.Next
	}
	return head.Next
}

func ToArr(list *ListNode) []int {
	var res []int
	for list != nil {
		res = append(res, list.Val)
		list = list.Next
	}
	return res
}
