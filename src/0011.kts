class Solution {
    fun maxArea(heights: IntArray): Int {
        var L = 0
        var R = heights.size - 1
        var result = 0
        while (L != R) {
            result = maxOf(result, (R - L) * minOf(heights[L], heights[R]))
            if (heights[L] < heights[R]) {
                L++
            } else {
                R--
            }
        }
        return result
    }
}