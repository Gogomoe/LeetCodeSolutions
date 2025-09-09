package main

import "fmt"

/**
 * Definition for singly-linked list.
 * type ListNode struct {
 *     Val int
 *     Next *ListNode
 * }
 */
func sortList(head *ListNode) *ListNode {
	if head == nil || head.Next == nil {
		return head
	}
	myHead := &ListNode{
		Next: head,
	}
	step := 1
	pre := myHead
	l1, l2, post := breakNextList(myHead, step)
	for l2 != nil {
		first, last := mergeTwoListsWithLast(l1, l2)
		pre.Next = first
		last.Next = post

		pre = last
		l1, l2, post = breakNextList(pre, step)
		if l2 == nil {
			step *= 2
			pre = myHead
			l1, l2, post = breakNextList(pre, step)
		}
	}

	return myHead.Next
}

func breakNextList(pre *ListNode, step int) (*ListNode, *ListNode, *ListNode) {
	l1 := pre.Next
	if l1 == nil {
		return l1, nil, nil
	}
	curr := l1
	for range step - 1 {
		curr = curr.Next
		if curr == nil {
			return l1, nil, nil
		}
	}
	l2 := curr.Next
	if l2 == nil {
		return l1, nil, nil
	}
	curr.Next = nil
	curr = l2
	for range step - 1 {
		curr = curr.Next
		if curr == nil {
			return l1, l2, nil
		}
	}
	post := curr.Next
	curr.Next = nil
	return l1, l2, post
}

func mergeTwoListsWithLast(list1 *ListNode, list2 *ListNode) (*ListNode, *ListNode) {
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

	return head.Next, last
}

func main() {
	res := sortList(ToList([]int{4, 2, 1, 3}))
	fmt.Printf("%v", ToArr(res))
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
