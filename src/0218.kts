import java.util.PriorityQueue
import java.util.ArrayDeque

class Solution {
    fun getSkyline(buildings: Array<IntArray>): List<List<Int>> {
        val pointList = mutableListOf<Triple<Boolean, Int, Int>>()
        for ((x1, x2, y) in buildings) {
            pointList.add(Triple(true, x1, y))
            pointList.add(Triple(false, x2, y))
        }
        val points = ArrayDeque(pointList.sortedWith(Comparator<Triple<Boolean, Int, Int>> { (_, x1, y1), (_, x2, y2) ->
            if (x1 != x2) {
                x1 - x2
            } else {
                y1 - y2
            }
        }))

        val pq = PriorityQueue<Int> { a, b -> -(a - b) }
        pq.add(0)
        var lastHeight = 0
        val result = mutableListOf<List<Int>>()

        while (points.isNotEmpty()) {
            val x = points.first().second
            while (points.isNotEmpty() && points.first().second == x) {
                val (isStart, _, y) = points.removeFirst()
                if (isStart) {
                    pq.add(y)
                } else {
                    pq.remove(y)
                }
            }

            val currHeight = pq.peek()
            if (currHeight != lastHeight) {
                result.add(listOf(x, currHeight))
                lastHeight = currHeight
            }
        }

        return result
    }
}

println(
    Solution().getSkyline(
        arrayOf(
            intArrayOf(2, 9, 10),
            intArrayOf(3, 7, 15),
            intArrayOf(5, 12, 12),
            intArrayOf(15, 20, 10),
            intArrayOf(19, 24, 8)
        )
    )
)
// [[2,10],[3,15],[7,12],[12,0],[15,10],[20,8],[24,0]]

println(
    Solution().getSkyline(
        arrayOf(
            intArrayOf(0, 2, 3),
            intArrayOf(2, 5, 3),
        )
    )
)
// [[0,3],[5,0]]