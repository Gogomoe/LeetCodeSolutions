import kotlin.math.max

class Solution {
    fun maximalRectangle(matrix: Array<CharArray>): Int {
        if (matrix.isEmpty() || matrix[0].isEmpty()) {
            return 0
        }
        val rows = matrix.size
        val cols = matrix[0].size
        val sums = Array(rows) { IntArray(cols) }
        for (c in 0 until cols) {
            var sum = 0
            for (r in 0 until rows) {
                if (matrix[r][c] == '1') {
                    sum += 1
                } else {
                    sum = 0
                }
                sums[r][c] = sum
            }
        }

        var result = 0
        for (r in 0 until rows) {
            result = max(result, largestRectangleArea(sums[r]))
        }

        return result
    }

    // from 0084
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

println(
    Solution().maximalRectangle(
        arrayOf(
            charArrayOf('1', '0', '1', '0', '0'),
            charArrayOf('1', '0', '1', '1', '1'),
            charArrayOf('1', '1', '1', '1', '1'),
            charArrayOf('1', '0', '0', '1', '0')
        )
    )
) // 6
println(Solution().maximalRectangle(emptyArray())) // 0
println(Solution().maximalRectangle(arrayOf(charArrayOf('0')))) // 0
println(Solution().maximalRectangle(arrayOf(charArrayOf('1')))) // 1
println(Solution().maximalRectangle(arrayOf(charArrayOf('0', '0')))) // 0