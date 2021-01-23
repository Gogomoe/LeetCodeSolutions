class Solution {
    fun removeDuplicates(nums: IntArray): Int {
        if (nums.isEmpty()) {
            return 0
        }
        var last = nums[0]
        var count = 0
        var store = 0

        for (i in nums.indices) {
            val it = nums[i]

            if (it == last) {
                count++
                if (count <= 2) {
                    nums[store++] = it
                }
            } else {
                last = it
                count = 1
                nums[store++] = it
            }
        }
        return store
    }
}