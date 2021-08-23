import java.util.*
import kotlin.Pair

class Node(val id: Int) {
    val adj: MutableList<Pair<Node, Int>> = mutableListOf()
}

class Solution {

    fun networkDelayTime(times: Array<IntArray>, N: Int, K: Int): Int {
        val nodes = Array(N) { Node(it) }
        for ((u, v, t) in times) {
            nodes[u - 1].adj.add(nodes[v - 1] to t)
        }

        val queue: PriorityQueue<Pair<Node, Int>> = PriorityQueue { o1, o2 -> o1.second - o2.second }
        val distTo = IntArray(N) { Int.MAX_VALUE }

        queue.add(nodes[K - 1] to 0)
        distTo[K - 1] = 0

        while (queue.isNotEmpty()) {
            val (node, _) = queue.poll()

            for ((adj, cost) in node.adj) {
                val oldDist = distTo[adj.id]
                val newDist = distTo[node.id] + cost
                if (distTo[adj.id] == Int.MAX_VALUE || newDist < oldDist) {
                    distTo[adj.id] = newDist
                    queue.remove(adj to oldDist)
                    queue.add(adj to newDist)
                }

            }
        }

        if (distTo.any { it == Int.MAX_VALUE }) return -1
        return distTo.maxOrNull()!!
    }
}