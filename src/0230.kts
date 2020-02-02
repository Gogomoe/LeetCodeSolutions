import kotlin.Pair

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
    fun kthSmallest(root: TreeNode?, k: Int): Int {
        return root.kth(k).first!!.`val`
    }

    private fun TreeNode?.kth(k: Int): Pair<TreeNode?, Int> {
        if (this == null) {
            return null to 0
        }
        val l = this.left.kth(k)
        if (l.first != null) return l
        if (l.second == k - 1) return this to k
        val r = this.right.kth(k - l.second - 1)
        return r.first to l.second + r.second + 1
    }
}

