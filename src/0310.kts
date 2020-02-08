import kotlin.Pair

class Solution {
    class Node(val id: Int) {
        val adj = mutableListOf<Node>()
        var color = -1

        fun findFurthest(from: Node?): Pair<Node, Int> {
            var maxNode = this
            var max = 0
            for (node in adj) {
                if (node == from) {
                    continue
                }
                val (itNode, it) = node.findFurthest(this)
                if (it > max) {
                    maxNode = itNode
                    max = it
                }
            }
            this.color = maxNode.color
            return maxNode to max + 1
        }
    }

    fun findMinHeightTrees(n: Int, edges: Array<IntArray>): List<Int> {
        if (n <= 2) {
            return (0 until n).toList()
        }
        val graph = Array(n) { Node(it) }
        for ((a, b) in edges) {
            graph[a].adj.add(graph[b])
            graph[b].adj.add(graph[a])
        }
        val (start, _) = graph[0].findFurthest(null)
        val (end, len) = start.findFurthest(null)

        var a = start
        var b = end
        val color = a.color
        repeat(len / 2) {
            a.color = -2
            b.color = -2
            a = a.adj.first { it.color == color }
            b = b.adj.first { it.color == color }
        }
        return if (n % 2 == 0) {
            listOf(a.id)
        } else {
            listOf(a.id, b.id)
        }
    }
}