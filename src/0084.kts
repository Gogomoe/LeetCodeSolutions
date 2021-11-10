import kotlin.math.max

class Solution {
    fun largestRectangleArea(heights: IntArray): Int {
        val len = heights.size

        val left = IntArray(len)
        val stack = ArrayDeque<Int>()
        for (i in heights.indices) {
            val it = heights[i]
            while (stack.isNotEmpty() && heights[stack.last()] >= it) {
                stack.removeLast()
            }
            if (stack.isEmpty()) {
                left[i] = 0
            } else {
                left[i] = stack.last() + 1
            }
            stack.addLast(i)
        }

        val right = IntArray(len)
        stack.clear()
        for (i in heights.indices.reversed()) {
            val it = heights[i]
            while (stack.isNotEmpty() && heights[stack.last()] >= it) {
                stack.removeLast()
            }
            if (stack.isEmpty()) {
                right[i] = len - 1
            } else {
                right[i] = stack.last() - 1
            }
            stack.addLast(i)
        }

        var result = 0
        for (i in heights.indices) {
            result = max(result, heights[i] * (right[i] - left[i] + 1))
        }

        return result
    }
}

println(Solution().largestRectangleArea(intArrayOf(2, 1, 5, 6, 2, 3))) // 10
println(Solution().largestRectangleArea(intArrayOf(2, 4))) // 4