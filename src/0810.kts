class Solution {
    fun xorGame(nums: IntArray): Boolean {
        return if (nums.reduce(Int::xor) == 0) {
            true
        } else {
            nums.size % 2 == 0
        }
    }
}