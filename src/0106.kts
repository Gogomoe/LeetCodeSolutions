/**
 * Example:
 * var ti = TreeNode(5)
 */

data class TreeNode(var `val`: Int, var left: TreeNode? = null, var right: TreeNode? = null)

class Solution {
    fun buildTree(inorder: IntArray, postorder: IntArray): TreeNode? {
        return buildTree(inorder, postorder, 0, postorder.size - 1, inorder.size)
    }

    fun buildTree(inorder: IntArray, postorder: IntArray, inIdx: Int, postIdx: Int, len: Int): TreeNode? {
        if (len <= 0) {
            return null
        }
        val value = postorder[postIdx]
        var idx = inIdx
        while (inorder[idx] != value) {
            idx += 1
        }
        val node = TreeNode(value)
        val leftLen = idx - inIdx
        val rightLen = len - leftLen - 1
        node.left = buildTree(inorder, postorder, idx - leftLen, postIdx - rightLen - 1, leftLen)
        node.right = buildTree(inorder, postorder, idx + 1, postIdx - 1, rightLen)
        return node
    }
}

println(Solution().buildTree(intArrayOf(9, 3, 15, 20, 7), intArrayOf(9, 15, 7, 20, 3))) // [3,9,20,null,null,15,7]
println(Solution().buildTree(intArrayOf(-1), intArrayOf(-1))) // [-1]