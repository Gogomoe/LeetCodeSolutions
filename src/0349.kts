class Solution {
    fun intersection(nums1: IntArray, nums2: IntArray): IntArray {
        val set = mutableSetOf(*nums1.toTypedArray())
        return nums2.filter { set.contains(it) }.distinct().toIntArray()
    }
}