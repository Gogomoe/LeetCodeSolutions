import kotlin.math.ceil
import kotlin.math.max
import kotlin.math.min

class Solution {
    fun maximumGap(nums: IntArray): Int {
        val n = nums.size
        if (n < 2) return 0
        val min = nums.minOrNull()!!
        val max = nums.maxOrNull()!!
        if (min == max) return 0
        if (n == 2) return max - min

        val gap = ceil((max - min).toDouble() / (n - 1)).toInt()
        val buckets = Array(n - 1) {
            IntArray(2) { i -> listOf(Int.MAX_VALUE, Int.MIN_VALUE)[i] }
        }
        for (i in nums) {
            val bucketId = if (i == max) n - 2 else (i - min) / gap
            val bucket = buckets[bucketId]
            bucket[0] = min(bucket[0], i)
            bucket[1] = max(bucket[1], i)
        }
        var maxGap = 0
        var last = buckets[0]
        for (i in 1 until buckets.size) {
            val it = buckets[i]
            if (it[0] == Int.MAX_VALUE && it[1] == Int.MIN_VALUE) {
                continue
            }
            maxGap = max(maxGap, it[0] - last[1])
            last = it
        }
        return maxGap
    }
}