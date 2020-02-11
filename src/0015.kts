class Solution {
    fun threeSum(nums: IntArray): List<List<Int>> {
        if (nums.size < 3) {
            return emptyList()
        }
        nums.sort()
        val res = mutableListOf<List<Int>>()
        var i = 0
        while (i < nums.size && nums[i] <= 0) {
            if (i - 1 >= 0 && nums[i] == nums[i - 1]) {
                i++
                continue
            }
            var k = nums.size - 1
            var j = i + 1
            while (j < k) {
                val it = nums[i] + nums[j] + nums[k]
                when {
                    it == 0 -> {
                        res.add(listOf(nums[i], nums[j], nums[k]))
                        while (j < k && nums[j + 1] == nums[j]) {
                            j++
                        }
                        while (j < k && nums[k - 1] == nums[k]) {
                            k--
                        }
                        j++
                        k--
                    }
                    it > 0 -> k--
                    else -> j++
                }
            }
            i++
        }
        return res
    }
}

