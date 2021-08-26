import kotlin.collections.HashMap
import kotlin.collections.HashSet
import kotlin.math.abs
import kotlin.math.min

class Solution {
    fun minAreaRect(points: Array<IntArray>): Int {
        val xs = HashMap<Int, HashSet<IntArray>>()
        val ys = HashMap<Int, HashSet<IntArray>>()
        val ps = HashMap<Int, HashSet<Int>>()

        for (it in points) {
            xs.putIfAbsent(it[0], HashSet())
            ys.putIfAbsent(it[1], HashSet())
            ps.putIfAbsent(it[0], HashSet())
            xs[it[0]]!!.add(it)
            ys[it[1]]!!.add(it)
            ps[it[0]]!!.add(it[1])
        }

        var res = Int.MAX_VALUE
        for (it in points) {
            val x1 = it[0]
            val y1 = it[1]
            for (xp in xs[x1]!!) {
                for (yp in ys[y1]!!) {
                    if (xp === it || yp === it) {
                        continue
                    }
                    val x2 = yp[0]
                    val y2 = xp[1]
                    if (ps[x2]?.contains(y2) == true) {
                        res = min(res, abs((x2 - x1) * (y2 - y1)))
                    }
                }
            }
            xs[x1]!!.remove(it)
            ys[y1]!!.remove(it)
        }

        return if (res == Int.MAX_VALUE) 0 else res
    }
}

println(
    Solution().minAreaRect(
        arrayOf(
            intArrayOf(1, 1),
            intArrayOf(1, 3),
            intArrayOf(3, 1),
            intArrayOf(3, 3),
            intArrayOf(2, 2),
        )
    )
)

println(
    Solution().minAreaRect(
        arrayOf(
            intArrayOf(1, 1),
            intArrayOf(1, 3),
            intArrayOf(3, 1),
            intArrayOf(3, 3),
            intArrayOf(4, 1),
            intArrayOf(4, 3),
        )
    )
)