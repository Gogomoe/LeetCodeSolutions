/**
 * Example:
 * var ti = Node(5)
 * var v = ti.`val`
 * Definition for a Node.
 * class Node(var `val`: Int) {
 *     var next: Node? = null
 *     var random: Node? = null
 * }
 */

class Node(var `val`: Int) {
    var next: Node? = null
    var random: Node? = null
}

class Solution {
    fun copyRandomList(node: Node?): Node? {
        if (node == null) {
            return null
        }

        val map = mutableMapOf<Node, Int>()
        val list = mutableListOf<Node>()
        var current = node
        while (current != null) {
            map.putIfAbsent(current, list.size)
            list.add(current)

            current = current.next
        }

        val result = List(list.size) { Node(list[it].`val`) }
        for (i in result.indices) {
            if (i != 0) {
                result[i - 1].next = result[i]
            }

            result[i].random = if (list[i].random == null) {
                null
            } else {
                result[map[list[i].random!!]!!]
            }
        }
        return result[0]
    }
}