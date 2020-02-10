import kotlin.math.max

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

    private val map = mutableMapOf<TreeNode, Int>()
    fun rob(root: TreeNode?): Int {
        return maxRob(root)
    }

    private fun maxRob(root: TreeNode?): Int {
        if (root == null) {
            return 0
        }
        if (map.containsKey(root)) {
            return map[root]!!
        }
        val res = max(
            maxRob(root.left) + maxRob(root.right),
            root.`val` + maxRob(root.left?.left) + maxRob(root.left?.right)
                    + maxRob(root.right?.left) + maxRob(root.right?.right)
        )
        map[root] = res
        return res
    }
}