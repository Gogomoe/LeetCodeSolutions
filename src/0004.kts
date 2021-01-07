class Solution {
    fun findMedianSortedArrays(nums1: IntArray, nums2: IntArray): Double {

        val (a, b) = if (nums1.size >= nums2.size) nums1 to nums2 else nums2 to nums1

        val size = a.size + b.size
        val halfSize = size / 2

        if (size == 0) {
            throw IllegalArgumentException("cannot calculate median of two empty arrays")
        }

        var L = 0
        var R = halfSize

        var i = (L + R) / 2
        var j = halfSize - i

        while (L < R) {
            if (j > b.size) {
                L = i + 1
                i = (L + R) / 2
                j = halfSize - i
                continue
            }
            if ((i == a.size || j == 0 || b[j - 1] <= a[i]) &&
                    (j == b.size || i == 0 || a[i - 1] <= b[j])) {
                break
            }
            if (b[j - 1] > a[i]) {
                L = i + 1
            } else {
                R = i
            }
            i = (L + R) / 2
            j = halfSize - i
        }

        val minOfRight = (when {
            i == a.size -> b[j]
            j == b.size -> a[i]
            else -> minOf(a[i], b[j])
        }).toDouble()

        return if (size % 2 == 1) {
            minOfRight
        } else {
            val maxOfLeft = (when {
                i == 0 -> b[j - 1]
                j == 0 -> a[i - 1]
                else -> maxOf(a[i - 1], b[j - 1])
            }).toDouble()
            return (minOfRight + maxOfLeft) / 2
        }

    }
}