import kotlin.math.max
import kotlin.math.min

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
    fun maxSumBST(root: TreeNode?): Int {
        return dfs(root).first
    }

    private fun dfs(root: TreeNode?): Triple<Int, Int?, Pair<Int, Int>?> {
        if (root == null) {
            return Triple(0, 0, null)
        }
        val (leftRes, leftSum, leftBounds) = dfs(root.left)
        val (rightRes, rightSum, rightBounds) = dfs(root.right)

        val sum = if (leftSum != null && rightSum != null &&
                (leftBounds == null || leftBounds.second < root.`val`) &&
                (rightBounds == null || root.`val` < rightBounds.first)) {
            leftSum + rightSum + root.`val`
        } else {
            null
        }

        val res = maxOf(0, leftRes, rightRes, sum)
        val bounds = Pair(
                minOf(root.`val`, leftBounds?.first, rightBounds?.first),
                maxOf(root.`val`, leftBounds?.second, rightBounds?.second)
        )

        return Triple(res, sum, bounds)
    }

    private fun maxOf(a: Int, vararg args: Int?): Int {
        var res = a
        for (it in args) {
            if (it != null) {
                res = max(res, it)
            }
        }
        return res
    }

    private fun minOf(a: Int, vararg args: Int?): Int {
        var res = a
        for (it in args) {
            if (it != null) {
                res = min(res, it)
            }
        }
        return res
    }

}