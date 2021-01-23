class Solution {
    fun canPartitionKSubsets(nums: IntArray, k: Int): Boolean {
        if (nums.size < k) {
            return false
        }
        val sum = nums.sum()
        if (sum % k != 0) {
            return false
        }
        val target = sum / k
        val sums = IntArray(k)

        return dfs(0, nums, sums, k, target)
    }

    private fun dfs(i: Int, nums: IntArray, sums: IntArray, k: Int, target: Int): Boolean {
        if (i == nums.size) {
            return true
        }
        val num = nums[i]
        for (group in 0 until k) {
            if (sums[group] + num <= target) {
                sums[group] += num
                val result = dfs(i + 1, nums, sums, k, target)
                if (result) {
                    return true
                }
                sums[group] -= num
            }
        }
        return false
    }
}