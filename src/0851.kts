class Solution {
    class Node(val id: Int, val quiet: Int) {
        val ins = mutableListOf<Node>()
        val outs = mutableListOf<Node>()
        var answer = this
    }

    fun loudAndRich(richer: Array<IntArray>, quiet: IntArray): IntArray {
        val nodes = Array(quiet.size) { Node(it, quiet[it]) }
        for ((u, v) in richer) {
            nodes[u].outs.add(nodes[v])
            nodes[v].ins.add(nodes[u])
        }

        val deque = ArrayDeque<Node>()

        for (node in nodes) {
            if (node.ins.isEmpty()) {
                deque.addLast(node)
            }
        }

        while (deque.isNotEmpty()) {
            val node = deque.removeFirst()
            for (next in node.outs) {
                if (node.answer.quiet < next.answer.quiet) {
                    next.answer = node.answer
                }
                next.ins.remove(node)
                if (next.ins.isEmpty()) {
                    deque.addLast(next)
                }
            }
        }

        return nodes.map { it.answer.id }.toIntArray()
    }
}