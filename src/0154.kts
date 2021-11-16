import kotlin.math.min

class Solution {
    fun findMin(nums: IntArray): Int {
        return findMinInRange(nums, 0, nums.size - 1);
    }

    private fun findMinInRange(nums: IntArray, l: Int, r: Int): Int {
        if (r - l <= 1) {
            return min(nums[l], nums[r]);
        }

        var res = nums[l]
        val mid = (l + r) / 2
        if (nums[mid] >= nums[r]) {
            res = min(res, findMinInRange(nums, mid, r))
        }
        if (nums[mid] <= nums[l]) {
            res = min(res, findMinInRange(nums, l, mid))
        }
        return res
    }
}

println(Solution().findMin(intArrayOf(1, 3, 5))) // 1
println(Solution().findMin(intArrayOf(2, 2, 2, 0, 1))) // 0
println(Solution().findMin(intArrayOf(3, 1, 3))) // 1
println(Solution().findMin(intArrayOf(3, 3, 1, 3))) // 1
println(Solution().findMin(intArrayOf(10, 1, 10, 10, 10))) // 1