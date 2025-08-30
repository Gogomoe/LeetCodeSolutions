package main

import (
	"fmt"
	"slices"
)

/**
 * Definition for singly-linked list.
 * type ListNode struct {
 *     Val int
 *     Next *ListNode
 * }
 */
func mergeTwoLists(list1 *ListNode, list2 *ListNode) *ListNode {
	head := &ListNode{}
	last := head

	for list1 != nil && list2 != nil {
		if list1.Val < list2.Val {
			last.Next = list1
			list1 = list1.Next
		} else {
			last.Next = list2
			list2 = list2.Next
		}
		last = last.Next
	}
	for list1 != nil {
		last.Next = list1
		list1 = list1.Next
		last = last.Next
	}
	for list2 != nil {
		last.Next = list2
		list2 = list2.Next
		last = last.Next
	}

	return head.Next
}

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

func main() {
	fmt.Println(slices.Equal(
		ToArr(mergeTwoLists(ToList([]int{1, 2, 4}), ToList([]int{1, 3, 4}))),
		[]int{1, 1, 2, 3, 4, 4},
	))
}
