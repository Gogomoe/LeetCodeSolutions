class Solution {
    fun uniquePathsWithObstacles(obstacleGrid: Array<IntArray>): Int {
        if (obstacleGrid.isEmpty() || obstacleGrid[0].isEmpty()) {
            return 0
        }
        val height = obstacleGrid.size
        val width = obstacleGrid[0].size
        val map = Array(height) { IntArray(width) }
        for (i in obstacleGrid.indices) {
            for (j in obstacleGrid[i].indices) {
                map[i][j] = when {
                    obstacleGrid[i][j] == 1 -> 0
                    i == 0 && j == 0 -> 1
                    i == 0 -> map[i][j - 1]
                    j == 0 -> map[i - 1][j]
                    else -> map[i][j - 1] + map[i - 1][j]
                }
            }
        }
        return map.last().last()
    }
}
