import kotlin.math.abs

class Solution {
    fun tallestBillboard(rods: IntArray): Int {
        var arr = IntArray(5001) { -10000 }
        var next = IntArray(5001) { -10000 }
        arr[0] = 0
        for (it in rods) {
            for (i in next.indices) {
                next[i] = arr[i]
                if (it > i) {
                    next[i] = maxOf(next[i], arr[-(i - it)] - (i - it))
                } else {
                    next[i] = maxOf(next[i], arr[i - it])
                }
                if (i + it <= 5000) {
                    next[i] = maxOf(next[i], arr[i + it] + it)
                }
            }
            val temp = arr
            arr = next
            next = temp
        }
        return arr[0]
    }
}