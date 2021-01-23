import java.lang.IllegalStateException

class Solution {
    fun findRedundantConnection(edges: Array<IntArray>): IntArray {
        val nodes = edges.size
        val set = UnionFindSet(nodes)
        for (it in edges) {
            val u = it[0] - 1
            val v = it[1] - 1
            if (!set.connected(u, v)) {
                set.union(u, v)
            } else {
                return it
            }
        }
        throw IllegalStateException("cannot reach this")
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