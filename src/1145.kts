class TreeNode(var `val`: Int) {
    var left: TreeNode? = null
    var right: TreeNode? = null
}

class Solution {
    fun btreeGameWinningMove(root: TreeNode?, n: Int, x: Int): Boolean {
        val xNode = findNode(root, x)!!
        val left = countTree(xNode.left)
        val right = countTree(xNode.right)
        return intArrayOf(left, right, n - 1 - left - right).any { it > n / 2 }
    }

    private fun countTree(root: TreeNode?): Int {
        if (root == null) {
            return 0
        }
        return countTree(root.left) + countTree(root.right) + 1
    }

    private fun findNode(root: TreeNode?, x: Int): TreeNode? {
        if (root == null) {
            return null
        }
        if (root.`val` == x) {
            return root
        }
        return findNode(root.left, x) ?: findNode(root.right, x)
    }
}