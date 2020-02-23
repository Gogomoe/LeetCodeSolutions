import java.lang.IllegalStateException

class Solution {
    fun findRedundantDirectedConnection(edges: Array<IntArray>): IntArray {
        val n = edges.size
        val parent = IntArray(n + 1) { -1 }

        val conflictEdge = mutableListOf<IntArray>()
        for (edge in edges) {
            val u = edge[0]
            val v = edge[1]

            if (parent[v] != -1) {
                conflictEdge.add(intArrayOf(parent[v], v))
                conflictEdge.add(intArrayOf(u, v))
            }

            parent[v] = u
        }

        if (conflictEdge.size == 2) {
            val p1 = conflictEdge[0][0]
            val p2 = conflictEdge[1][0]
            val c = conflictEdge[0][1]

            fun findRoot(num: Int): Int {
                var it = num
                while (parent[it] != -1 && it != c) {
                    it = parent[it]
                }
                return it
            }

            val r1 = findRoot(p1)
            if (r1 == c) {
                return intArrayOf(p1, c)
            }
            return intArrayOf(p2, c)

        } else {
            val uns = UnionFindSet(n + 1)

            for (edge in edges) {
                val u = edge[0]
                val v = edge[1]

                if (uns.find(u) == v) {
                    return edge
                }

                uns.union(v, u)
            }
            throw IllegalStateException()
        }

    }

    class UnionFindSet(var n: Int) {
        var parents: IntArray = IntArray(n)
        fun find(it: Int): Int {
            if (parents[it] == it) {
                return it
            }
            val parent = find(parents[it])
            parents[it] = parent
            return parent
        }

        fun connected(a: Int, b: Int): Boolean {
            return find(a) == find(b)
        }

        fun union(a: Int, b: Int) {
            val pa = find(a)
            val pb = find(b)
            if (pa == pb) {
                return
            }
            parents[pa] = pb
        }

        init {
            for (i in 0 until n) {
                parents[i] = i
            }
        }
    }

}