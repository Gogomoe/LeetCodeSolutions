/**
 * Example:
 * var ti = TreeNode(5)
 * var v = ti.`val`
 * Definition for a binary tree node.
 */
data class TreeNode(var `val`: Int, var left: TreeNode? = null, var right: TreeNode? = null)

class Solution {
    fun levelOrder(root: TreeNode?): List<List<Int>> {
        var curr = ArrayDeque<TreeNode>()
        var next = ArrayDeque<TreeNode>()
        val result = mutableListOf<List<Int>>()
        root?.let { next.addLast(it) }
        while (next.isNotEmpty()) {
            val temp = curr
            curr = next
            next = temp

            val level = mutableListOf<Int>()
            while (curr.isNotEmpty()) {
                val node = curr.removeFirst()
                level.add(node.`val`)
                node.left?.let { next.addLast(it) }
                node.right?.let { next.addLast(it) }
            }

            result.add(level)
        }
        return result
    }
}

println(
    Solution().levelOrder(
        TreeNode(
            3,
            TreeNode(9),
            TreeNode(
                20,
                TreeNode(15),
                TreeNode(7)
            )
        )
    )
) // [[3],[9,20],[15,7]]

println(
    Solution().levelOrder(
        TreeNode(
            1
        )
    )
) // [[1]]

println(
    Solution().levelOrder(
        null
    )
) // []