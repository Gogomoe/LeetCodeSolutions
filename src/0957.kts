import java.util.*

class Solution {
    fun prisonAfterNDays(cells: IntArray, N: Int): IntArray {
        val last = cells.clone()
        val next = cells.clone()
        val map = mutableMapOf<Int, Int>()

        var num = mapToInt(next)
        map[num] = 0

        repeat(N) { index ->
            calculateNext(next, last)

            num = mapToInt(next)
            if (map.keys.contains(num)) {
                val start = map[num]!!
                val loopLength = (index + 1) - start
                val remain = (N - index - 1) % loopLength

                repeat(remain) {
                    calculateNext(next, last)
                }
                return next
            } else {
                map[num] = index + 1
            }

        }
        return next
    }

    private fun calculateNext(next: IntArray, last: IntArray) {
        for (i in 0..7) {
            last[i] = next[i]
        }
        for (i in 1..6) {
            next[i] = if (last[i - 1] == last[i + 1]) 1 else 0
        }
        next[0] = 0
        next[7] = 0
    }

    private fun mapToInt(next: IntArray): Int {
        var num = 0
        for (i in 0..7) {
            num += next[i] shl i
        }
        return num
    }
}

println(Solution().prisonAfterNDays(intArrayOf(0, 1, 0, 1, 1, 0, 0, 1), 1000).contentToString())