class Solution {
    fun reversePairs(nums: IntArray): Int {
        return merge(nums, IntArray(nums.size), 0, nums.size)
    }

    private fun merge(nums: IntArray, buffer: IntArray, start: Int, end: Int): Int {
        if (end - start <= 1) {
            return 0
        }
        val mid = (start + end) / 2

        var res = merge(nums, buffer, start, mid) + merge(nums, buffer, mid, end)

        var l = start
        var r = mid
        while (l != mid) {
            while (r != end && nums[l] / 2.0 >  nums[r]) {
                r++
            }
            res += r - mid
            l++
        }

        l = start
        r = mid
        var i = start
        while (l != mid && r != end) {
            if (nums[l] > nums[r]) {
                buffer[i++] = nums[r++]
            } else {
                buffer[i++] = nums[l++]
            }
        }
        while (l != mid) {
            buffer[i++] = nums[l++]
        }
        while (r != end) {
            buffer[i++] = nums[r++]
        }
        System.arraycopy(buffer, start, nums, start, end - start)
        return res
    }
}
