class Node(var `val`: Int) {
    var left: Node? = null
    var right: Node? = null
    var next: Node? = null
}

class Solution {
    fun connect(root: Node?): Node? {
        if (root == null) {
            return root
        }
        var list = mutableListOf<Node>()
        list.add(root)

        while (list.isNotEmpty()) {
            val next = mutableListOf<Node>()
            for (i in 0..(list.size - 2)) {
                list[i].next = list[i + 1]
                list[i].left?.let { next.add(it) }
                list[i].right?.let { next.add(it) }
            }
            list.last().left?.let { next.add(it) }
            list.last().right?.let { next.add(it) }

            list = next
        }

        return root
    }
}