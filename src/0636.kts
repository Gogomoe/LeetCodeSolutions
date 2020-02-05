import java.util.*

class Solution {
    fun exclusiveTime(n: Int, logs: List<String>): IntArray {
        val res = IntArray(n)
        fun helper(queue: ArrayDeque<Triple<Int, Boolean, Int>>): Int {
            val (id, _, start) = queue.pollFirst()
            var subCost = 0
            while (queue.first.second) {
                subCost += helper(queue)
            }
            val (_, _, end) = queue.pollFirst()
            res[id] += end - start - subCost + 1
            return end - start + 1
        }

        val deque = ArrayDeque(logs.map {
            val (a, b, c) = it.split(':')
            Triple(a.toInt(), b == "start", c.toInt())
        })
        while (deque.isNotEmpty()) {
            helper(deque)
        }
        return res
    }

}
