import java.util.*

class Solution {
    fun oddEvenJumps(arr: IntArray): Int {
        val size = arr.size
        if (size <= 1) {
            return size
        }


        val map = TreeMap<Int, Int>() // arr[i] -> i
        map[arr[size - 1]] = size - 1

        // [0] even jump
        // [1] odd jump
        val reach = Array(2) { BooleanArray(size) { false } }
        reach[0][size - 1] = true
        reach[1][size - 1] = true

        var count = 1

        for (i in (size - 2) downTo 0) {
            val it = arr[i]

            val oddJump = map.ceilingEntry(it)
            if (oddJump != null) {
                reach[1][i] = reach[0][oddJump.value]
            }

            val evenJump = map.floorEntry(it)
            if (evenJump != null) {
                reach[0][i] = reach[1][evenJump.value]
            }

            if (reach[1][i]) {
                count++
            }

            map[it] = i
        }

        return count
    }
}

println(Solution().oddEvenJumps(intArrayOf(10, 13, 12, 14, 15)))
println(Solution().oddEvenJumps(intArrayOf(2, 3, 1, 1, 4)))
println(Solution().oddEvenJumps(intArrayOf(5, 1, 3, 4, 2)))
println(Solution().oddEvenJumps(intArrayOf(1, 2, 3, 2, 1, 4, 4, 5)))