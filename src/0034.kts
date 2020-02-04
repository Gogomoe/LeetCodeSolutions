class Solution {
    fun searchRange(nums: IntArray, target: Int): IntArray {
        return intArrayOf(findFirst(nums, target), findLast(nums, target))
    }

    private fun findLast(nums: IntArray, target: Int): Int {
        if (nums.isEmpty()) {
            return -1
        }
        var L = 0
        var R = nums.size - 1
        while (L < R) {
            val mid = L + (R - L + 1) / 2
            if (nums[mid] <= target) {
                L = mid
            } else {
                R = mid - 1
            }
        }
        return if (nums[L] == target) L else -1
    }

    private fun findFirst(nums: IntArray, target: Int): Int {
        if (nums.isEmpty()) {
            return -1
        }
        var L = 0
        var R = nums.size - 1
        while (L < R) {
            val mid = L + (R - L) / 2
            if (nums[mid] >= target) {
                R = mid
            } else {
                L = mid + 1
            }
        }
        return if (nums[L] == target) L else -1
    }
}