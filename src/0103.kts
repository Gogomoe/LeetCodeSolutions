/**
 * Example:
 * var ti = TreeNode(5)
 * var v = ti.`val`
 * Definition for a binary tree node.
 * class TreeNode(var `val`: Int) {
 *     var left: TreeNode? = null
 *     var right: TreeNode? = null
 * }
 */


class TreeNode(var `val`: Int) {
    var left: TreeNode? = null
    var right: TreeNode? = null
}

class Solution {
    fun zigzagLevelOrder(root: TreeNode?): List<List<Int>> {
        if (root == null) {
            return emptyList()
        }

        var right = true
        var queue = ArrayDeque<TreeNode>()
        var next = ArrayDeque<TreeNode>()
        val result = mutableListOf<List<Int>>()

        queue.addLast(root)

        while (queue.isNotEmpty()) {
            val intList = mutableListOf<Int>()
            while (queue.isNotEmpty()) {
                val it = queue.removeFirst()
                intList.add(it.`val`)
                if (right) {
                    it.left?.let { next.addFirst(it) }
                    it.right?.let { next.addFirst(it) }
                } else {
                    it.right?.let { next.addFirst(it) }
                    it.left?.let { next.addFirst(it) }
                }
            }

            result.add(intList)
            right = !right
            val temp = queue
            queue = next
            next = temp
        }

        return result
    }
}