class Solution {
    fun firstMissingPositive(nums: IntArray): Int {
        for (i in nums.indices) {
            while (true) {
                val value = nums[i]
                if (value < 1 || value > nums.size || value == i + 1) {
                    break
                }
                val newIndex = value - 1
                val newValue = nums[newIndex]
                if (value == newValue) {
                    break
                }
                nums[newIndex] = value
                nums[i] = newValue
            }
        }
        for (i in nums.indices) {
            if (nums[i] != i + 1) {
                return i + 1
            }
        }
        return nums.size + 1
    }

}