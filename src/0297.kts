import java.util.*

/**
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

class Codec() {
    // Encodes a URL to a shortened URL.
    fun serialize(root: TreeNode?): String {
        return "[${serializeNode(root).joinToString(",") { if (it == null) "null" else "${it.`val`}" }}]"
    }

    private fun serializeNode(root: TreeNode?): List<TreeNode?> {
        val result = mutableListOf<TreeNode?>()
        val queue = LinkedList<TreeNode?>()

        queue.addLast(root)
        while (queue.isNotEmpty()) {
            val node = queue.removeFirst()
            result.add(node)
            if (node != null) {
                queue.addLast(node.left)
                queue.addLast(node.right)
            }
        }

        while (result.isNotEmpty() && result.last() == null) {
            result.removeAt(result.size - 1)
        }

        return result
    }

    // Decodes your encoded data to tree.
    fun deserialize(data: String): TreeNode? {
        val list = data.removeSurrounding("[", "]")
                .split(",")
                .map { it.toIntOrNull() }

        if (list.size == 1 && list[0] == null) {
            return null
        }

        val queue = ArrayDeque<TreeNode>()
        val root = TreeNode(list.first()!!)
        queue.addLast(root)
        var i = 1

        while (queue.isNotEmpty()) {
            val node = queue.removeFirst()
            if (i != list.size) {
                val value = list[i]
                if (value != null) {
                    val newNode = TreeNode(value)
                    node.left = newNode
                    queue.addLast(newNode)
                }
                i++
            }
            if (i != list.size) {
                val value = list[i]
                if (value != null) {
                    val newNode = TreeNode(value)
                    node.right = newNode
                    queue.addLast(newNode)
                }
                i++
            }
        }
        return root
    }
}

/**
 * Your Codec object will be instantiated and called as such:
 * var ser = Codec()
 * var deser = Codec()
 * var data = ser.serialize(longUrl)
 * var ans = deser.deserialize(data)
 */

Codec().serialize(
        TreeNode(1).apply {
            left = TreeNode(2).apply { }
            right = TreeNode(3).apply {
                left = TreeNode(4)
                right = TreeNode(5)
            }
        }
)
Codec().serialize(
        null
)