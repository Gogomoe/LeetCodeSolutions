import kotlin.math.max

class Solution {
    fun constrainedSubsetSum(nums: IntArray, k: Int): Int {
        var result = Int.MIN_VALUE
        var deque = ArrayDeque<Pair<Int, Int>>()

        for (i in nums.indices) {
            val it = nums[i]

            while (deque.isNotEmpty() && i - deque.first().second > k) {
                deque.removeFirst()
            }

            val sum = it + ((deque.firstOrNull()?.first ?: 0).takeIf { it >= 0 } ?: 0)
            result = max(result, sum)

            while (deque.isNotEmpty() && deque.last().first < sum) {
                deque.removeLast()
            }

            deque.addLast(sum to i)
        }

        return result
    }
}