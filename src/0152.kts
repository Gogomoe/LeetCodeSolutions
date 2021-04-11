class Solution {
    fun maxProduct(nums: IntArray): Int {
        var max = nums[0]
        var curMax = max
        var curMin = max
        for (it in nums.drop(1)) {
            val nextMax = maxOf(curMax * it, curMin * it, it)
            val nextMin = minOf(curMax * it, curMin * it, it)
            curMax = nextMax
            curMin = nextMin
            max = maxOf(max, curMax)
        }

        return max
    }

}
