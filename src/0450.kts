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
    fun deleteNode(root: TreeNode?, key: Int): TreeNode? {
        if (root == null) {
            return null
        }

        fun findMin(node: TreeNode?): Int? {
            if (node == null) {
                return null
            }
            var it = node
            while (it!!.left != null) {
                it = it.left!!
            }
            return it.`val`
        }

        return when {
            root.`val` < key -> {
                root.right = deleteNode(root.right, key)
                root
            }
            root.`val` > key -> {
                root.left = deleteNode(root.left, key)
                root
            }
            else -> {
                val minKey = findMin(root.right)
                if (minKey == null) {
                    root.left
                } else {
                    val newRight = deleteNode(root.right, minKey)
                    return TreeNode(minKey).also {
                        it.left = root.left
                        it.right = newRight
                    }
                }
            }
        }
    }
}
