class Solution {
    fun numSubarrayBoundedMax(A: IntArray, L: Int, R: Int): Int {
        var lastNotSatisfy = -1
        var lastSatisfy = -1
        var result = 0
        for (i in A.indices) {
            val it = A[i]
            when {
                it < L -> {
                    if (lastSatisfy != -1) {
                        result += (lastSatisfy - lastNotSatisfy)
                    }
                }
                it <= R -> {
                    result += (i - lastNotSatisfy)
                    lastSatisfy = i
                }
                else -> {
                    lastNotSatisfy = i
                    lastSatisfy = -1
                }
            }
        }
        return result
    }
}