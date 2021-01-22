import java.util.*

class Solution {
    fun lengthOfLIS(nums: IntArray): Int {
        val tree = TreeMap<Int, Int>()
        for (it in nums) {
            val prev = tree.floorEntry(it - 1)
            val value = if (prev == null) {
                1
            } else {
                prev.value + 1
            }
            tree[it] = value

            var next = tree.ceilingEntry(it + 1)
            while (next != null && next.value <= value) {
                tree.remove(next.key)
                next = tree.ceilingEntry(it + 1)
            }
        }
        return tree.floorEntry(Int.MAX_VALUE)?.value ?: 0
    }
}
