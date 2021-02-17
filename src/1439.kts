import java.util.*

class Solution {
    fun kthSmallest(mat: Array<IntArray>, k: Int): Int {
        var pq = PriorityQueue<Int>(Collections.reverseOrder())
        pq.add(0)
        for (row in mat) {
            val next = PriorityQueue<Int>(Collections.reverseOrder())
            for (prev in pq) {
                for (cur in row) {
                    next.add(prev + cur)
                }
            }
            while (next.size > k) {
                next.poll()
            }
            pq = next
        }
        return pq.poll()
    }
}