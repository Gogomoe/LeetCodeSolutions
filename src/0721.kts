class Solution {
    fun accountsMerge(accounts: List<List<String>>): List<List<String>> {
        val map = mutableMapOf<String, Int>()
        val uf = UnionFindSet(accounts.size)

        for (i in accounts.indices) {
            val it = accounts[i]
            for (mail in it.asSequence().drop(1)) {
                if (mail in map) {
                    val j = map[mail]!!
                    uf.union(i, j)
                }
                map[mail] = i
            }
        }

        var count = 0
        val toOldIndex = IntArray(accounts.size)
        val toNewIndex = IntArray(accounts.size)

        for (i in accounts.indices) {
            if (uf.find(i) == i) {
                toOldIndex[count] = i
                toNewIndex[i] = count
                count++
            }
        }

        val names = List<String>(count) { accounts[toOldIndex[it]][0] }
        val mails = List<MutableSet<String>>(count) { mutableSetOf() }

        for (i in accounts.indices) {
            val it = accounts[i]
            mails[toNewIndex[uf.find(i)]].addAll(it.asSequence().drop(1))
        }

        return names.zip(mails).map {
            mutableListOf(it.first).apply { addAll(it.second.sorted()) }
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