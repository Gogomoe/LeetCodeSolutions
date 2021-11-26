/**
 * Example:
 * var ti = TreeNode(5)
 */

data class TreeNode(var `val`: Int, var left: TreeNode? = null, var right: TreeNode? = null)

class Solution {
    fun buildTree(preorder: IntArray, inorder: IntArray): TreeNode? {
        return buildTree(preorder, inorder, 0, 0, inorder.size)
    }

    fun buildTree(preorder: IntArray, inorder: IntArray, preIdx: Int, inIdx: Int, len: Int): TreeNode? {
        if (len <= 0) {
            return null
        }
        val value = preorder[preIdx]
        var idx = inIdx
        while (inorder[idx] != value) {
            idx += 1
        }
        val node = TreeNode(value)
        val leftLen = idx - inIdx
        val rightLen = len - leftLen - 1
        node.left = buildTree(preorder, inorder, preIdx + 1, idx - leftLen, leftLen)
        node.right = buildTree(preorder, inorder, preIdx + 1 + leftLen, idx + 1, rightLen)
        return node
    }
}

println(Solution().buildTree(intArrayOf(3, 9, 20, 15, 7), intArrayOf(9, 3, 15, 20, 7))) // [3,9,20,null,null,15,7]
println(Solution().buildTree(intArrayOf(-1), intArrayOf(-1))) // [-1]