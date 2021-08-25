import kotlin.math.min

class Solution {
    fun minCost(houses: IntArray, cost: Array<IntArray>, m: Int, n: Int, target: Int): Int {
        fun paintCost(i: Int, c: Int) = if (houses[i] == 0) cost[i][c - 1] else throw IllegalStateException()

        val dp = Array(m) {
            Array(target + 1) {
                IntArray(n + 1) { -1 }
            }
        }

        if (houses[0] == 0) {
            for (c in 1..n) {
                dp[0][1][c] = paintCost(0, c)
            }
        } else {
            dp[0][1][houses[0]] = 0
        }

        val list = mutableListOf<Int>()
        for (i in 1 until m) {
            for (t in 1..min(i + 1, target)) {
                if (houses[i] == 0) {
                    for (c in 1..n) {
                        list.clear()
                        list.add(dp[i - 1][t][c])
                        for (prevC in 1 until c) {
                            list.add(dp[i - 1][t - 1][prevC])
                        }
                        for (prevC in c + 1..n) {
                            list.add(dp[i - 1][t - 1][prevC])
                        }
                        val res = list.filter { it != -1 }.minOrNull()?.plus(paintCost(i, c))
                        dp[i][t][c] = res ?: -1
                    }
                } else {
                    val c = houses[i]
                    list.clear()
                    list.add(dp[i - 1][t][c])
                    for (prevC in 1 until c) {
                        list.add(dp[i - 1][t - 1][prevC])
                    }
                    for (prevC in c + 1..n) {
                        list.add(dp[i - 1][t - 1][prevC])
                    }
                    val res = list.filter { it != -1 }.minOrNull()
                    dp[i][t][c] = res ?: -1
                }

            }
        }

        return dp[m - 1][target].filter { it != -1 }.minOrNull() ?: -1
    }
}

println(
    Solution().minCost(
        intArrayOf(0, 0, 0, 0, 0),
        arrayOf(
            intArrayOf(1, 10),
            intArrayOf(10, 1),
            intArrayOf(10, 1),
            intArrayOf(1, 10),
            intArrayOf(5, 1)
        ),
        5,
        2,
        3
    )
)

println(
    Solution().minCost(
        intArrayOf(0, 2, 1, 2, 0),
        arrayOf(
            intArrayOf(1, 10),
            intArrayOf(10, 1),
            intArrayOf(10, 1),
            intArrayOf(1, 10),
            intArrayOf(5, 1)
        ),
        5,
        2,
        3
    )
)
println(
    Solution().minCost(
        intArrayOf(0, 0, 0, 0, 0),
        arrayOf(
            intArrayOf(1, 10),
            intArrayOf(10, 1),
            intArrayOf(1, 10),
            intArrayOf(10, 1),
            intArrayOf(1, 10)
        ),
        5,
        2,
        5
    )
)
println(
    Solution().minCost(
        intArrayOf(3, 1, 2, 3),
        arrayOf(
            intArrayOf(1, 1, 1),
            intArrayOf(1, 1, 1),
            intArrayOf(1, 1, 1),
            intArrayOf(1, 1, 1)
        ),
        4,
        3,
        3
    )
)