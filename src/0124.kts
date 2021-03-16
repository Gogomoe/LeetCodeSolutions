import kotlin.math.max

class TreeNode(var `val`: Int) {
    var left: TreeNode? = null
    var right: TreeNode? = null
}

class Solution {

    fun maxPathSum(root: TreeNode?): Int {
        return maxPathSumForNode(root!!).second
    }

    private fun maxPathSumForNode(node: TreeNode): Pair<Int, Int> {
        val (ls, lm) = node.left?.let { maxPathSumForNode(it) } ?: -10000 to -10000
        val (rs, rm) = node.right?.let { maxPathSumForNode(it) } ?: -10000 to -10000

        var sum = node.`val`
        sum = max(sum, ls + node.`val`)
        sum = max(sum, rs + node.`val`)

        val max = max(max(sum, ls + rs + node.`val`), max(lm, rm))

        return sum to max
    }

}