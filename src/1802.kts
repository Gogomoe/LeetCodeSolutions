import kotlin.math.min
import kotlin.math.sqrt

class Solution {
    fun maxValue(n: Int, index: Int, maxSum: Int): Int {
        val i = min(index, n - index - 1)

        val partLeft = i.toLong() * i + n
        val partRight = (n - i).toLong() * (n - i) - (n - 2 * i).toLong() * (n - 2 * i - 1) / 2

        return if (maxSum >= partRight) {
            ((n - i) + (maxSum - partRight) / n).toInt()
        } else if (maxSum >= partLeft) {
            val b = (4 * i + 1).toDouble()
            val c = (-2 * (maxSum - partLeft)).toDouble()
            (i + 1) + ((-b + sqrt(b * b - 4 * c)) / 2).toInt()
        } else {
            1 + sqrt((maxSum - n).toDouble()).toInt()
        }
    }
}

println(Solution().maxValue(4, 1, 4))
println(Solution().maxValue(4, 1, 5))
println(Solution().maxValue(4, 1, 6))
println(Solution().maxValue(4, 1, 7))
println(Solution().maxValue(4, 1, 8))
println(Solution().maxValue(4, 1, 9))
println(Solution().maxValue(4, 1, 10))
println(Solution().maxValue(4, 1, 11))
println(Solution().maxValue(4, 1, 12))

println(
    Solution().maxValue(
        4116541,
        2948244,
        392357701
    )
)