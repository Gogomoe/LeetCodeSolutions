import kotlin.math.max

class Solution {
    fun minSwaps(nums: IntArray): Int {
        val n = nums.size
        val count = nums.count { it == 1 }
        if (count == 0) {
            return 0
        }
        val deque = ArrayDeque<Int>()
        for (i in 0 until count) {
            deque.addLast(nums[i])
        }
        var ones = deque.count { it == 1 }
        var maxOnes = ones
        for (i in count until count + n) {
            if (deque.removeFirst() == 1) {
                ones--
            }
            val it = nums[i % n]
            deque.addLast(it)
            if (it == 1) {
                ones++
            }
            maxOnes = max(ones, maxOnes)
        }
        return count - maxOnes
    }
}

println(Solution().minSwaps(intArrayOf(0, 1, 0, 1, 1, 0, 0))) // 1
println(Solution().minSwaps(intArrayOf(0, 1, 1, 1, 0, 0, 1, 1, 0))) // 2
println(Solution().minSwaps(intArrayOf(1, 1, 0, 0, 1))) // 0