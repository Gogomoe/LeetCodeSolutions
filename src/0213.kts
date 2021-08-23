import kotlin.math.max

class Solution {
    fun rob(nums: IntArray): Int {
        val len = nums.size
        if (len <= 3) {
            return nums.maxOrNull() ?: 0
        }
        val fromStart = IntArray(len)
        val fromSecond = IntArray(len)
        fromStart[0] = nums[0]
        fromStart[1] = max(nums[0], nums[1])
        fromSecond[0] = 0
        fromSecond[1] = nums[1]

        var result = 0
        for (i in 2 until len) {
            if (i != len - 1) {
                fromStart[i] = max(fromStart[i - 2] + nums[i], fromStart[i - 1])
            }
            fromSecond[i] = max(fromSecond[i - 2] + nums[i], fromSecond[i - 1])
            result = max(result, fromStart[i])
            result = max(result, fromSecond[i])
        }
        return result
    }
}