class Solution {
    fun candy(ratings: IntArray): Int {
        if (ratings.size <= 1) {
            return ratings.size
        }

        val size = ratings.size
        var result = IntArray(size)
        var candy = 1
        for (i in 1 until size) {
            if (ratings[i] > ratings[i - 1]) {
                candy++
            } else {
                candy = 1
            }
            result[i] = maxOf(result[i], candy)
        }

        candy = 1
        for (i in (size - 2) downTo 0) {
            if (ratings[i] > ratings[i + 1]) {
                candy++
            } else {
                candy = 1
            }
            result[i] = maxOf(result[i], candy)
        }

        return result.sum()
    }
}
