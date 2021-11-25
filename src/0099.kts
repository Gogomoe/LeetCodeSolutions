/**
 * Example:
 * var ti = TreeNode(5)
 * var v = ti.`val`
 * Definition for a binary tree node.
 */
data class TreeNode(var `val`: Int, var left: TreeNode? = null, var right: TreeNode? = null)

class Solution {
    var first: TreeNode? = null
    var second: TreeNode? = null
    var prev: TreeNode? = null

    fun recoverTree(root: TreeNode?): Unit {
        traverse(root)
        val temp = first!!.`val`;
        first!!.`val` = second!!.`val`;
        second!!.`val` = temp;
    }

    private fun traverse(node: TreeNode?) {
        if (node == null) {
            return
        }

        traverse(node.left)
        if (first == null && node.`val` < (prev?.`val` ?: Int.MIN_VALUE)) {
            first = prev
            second = node
        }

        if (first != null && node.`val` < (prev?.`val` ?: Int.MIN_VALUE)) {
            second = node
        }

        prev = node
        traverse(node.right)
    }
}

val tree1 = TreeNode(
    1,
    TreeNode(
        3,
        null,
        TreeNode(2)
    ),
    null
)
Solution().recoverTree(tree1)
println(tree1) // [3,1,null,null,2]


val tree2 = TreeNode(
    3,
    TreeNode(
        1,
        null,
        null
    ),
    TreeNode(
        4,
        TreeNode(2),
        null
    )
)
Solution().recoverTree(tree2)
println(tree2) // [2,1,4,null,null,3]