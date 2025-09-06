package main

/**
 * Definition for a binary tree node.
 * type TreeNode struct {
 *     Val int
 *     Left *TreeNode
 *     Right *TreeNode
 * }
 */
func isSymmetric(root *TreeNode) bool {
	return isSymmetricNodes(root.Left, root.Right)
}

func isSymmetricNodes(left *TreeNode, right *TreeNode) bool {
	if left == nil && right == nil {
		return true
	}
	if left == nil || right == nil {
		return false
	}
	if left.Val != right.Val {
		return false
	}
	if !isSymmetricNodes(left.Left, right.Right) {
		return false
	}
	if !isSymmetricNodes(left.Right, right.Left) {
		return false
	}
	return true
}
