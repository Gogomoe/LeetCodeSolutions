import kotlin.math.max

class Solution {
    fun maxProfitAssignment(difficulty: IntArray, profit: IntArray, worker: IntArray): Int {
        val zip = difficulty.mapIndexed { index, diff -> diff to profit[index] }.sortedBy { it.first }
        worker.sort()

        var index = -1
        var profit = 0

        var sum = 0
        for (w in worker) {
            while (index < zip.size - 1 && zip[index + 1].first <= w) {
                index++
                profit = max(profit, zip[index].second)
            }
            sum += profit
        }
        return sum
    }
}