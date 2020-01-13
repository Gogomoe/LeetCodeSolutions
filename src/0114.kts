class TreeNode(var `val`: Int) {
    var left: TreeNode? = null
    var right: TreeNode? = null
}

class Solution {

    fun flatten(root: TreeNode?): Unit {
        root ?: return
        flattenTree(root)
    }

    private fun flattenTree(root: TreeNode): TreeNode {
        val (left, right) = root.left to root.right
        if (left == null && right == null) {
            return root
        } else if (left != null && right != null) {
            val leftTail = flattenTree(left)
            val rightTail = flattenTree(right)
            root.left = null
            root.right = left
            leftTail.right = right
            return rightTail
        } else {
            val subtree = left ?: right!!
            val subtreeTail = flattenTree(subtree)
            root.left = null
            root.right = subtree
            return subtreeTail
        }

    }
}