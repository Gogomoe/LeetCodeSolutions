class Solution {
    fun findMinArrowShots(points: Array<IntArray>): Int {
        points.sortBy { it[1] }
        var count = 0
        var i = 0
        while (i < points.size) {
            val end = points[i][1]
            count++
            while (i < points.size && points[i][0] <= end) {
                i++
            }
        }
        return count
    }
}