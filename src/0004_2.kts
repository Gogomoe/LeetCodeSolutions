class Solution {
    fun findMedianSortedArrays(nums1: IntArray, nums2: IntArray): Double {
        if (nums1.size > nums2.size) {
            return findMedianSortedArrays(nums2, nums1)
        }

        var L = 0
        var R = nums1.size

        while (L <= R) {
            val mid1 = (L + R) / 2
            val mid2 = (nums1.size + nums2.size) / 2 - mid1

            val L1 = if (mid1 == 0) Int.MIN_VALUE else nums1[mid1 - 1]
            val L2 = if (mid2 == 0) Int.MIN_VALUE else nums2[mid2 - 1]
            val R1 = if (mid1 == nums1.size) Int.MAX_VALUE else nums1[mid1]
            val R2 = if (mid2 == nums2.size) Int.MAX_VALUE else nums2[mid2]

            if (L1 > R2) {
                R = mid1
            } else if (L2 > R1) {
                L = mid1 + 1
            } else {
                return if ((nums1.size + nums2.size) % 2 == 1) {
                    minOf(R1, R2).toDouble()
                } else {
                    (maxOf(L1, L2) + minOf(R1, R2)).toDouble() / 2.0
                }
            }
        }
        return -1.0
    }
}

println(Solution().findMedianSortedArrays(intArrayOf(1, 3), intArrayOf(2))) // 2
println(Solution().findMedianSortedArrays(intArrayOf(1, 2), intArrayOf(3, 4))) // 2.5
println(Solution().findMedianSortedArrays(intArrayOf(0, 0), intArrayOf(0, 0))) // 0
println(Solution().findMedianSortedArrays(intArrayOf(), intArrayOf(1))) // 1
println(Solution().findMedianSortedArrays(intArrayOf(2), intArrayOf())) // 2
println(Solution().findMedianSortedArrays(intArrayOf(2), intArrayOf(1, 3, 4, 5))) // 3