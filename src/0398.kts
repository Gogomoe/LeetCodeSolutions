import kotlin.random.Random

class Solution(val nums: IntArray) {

    fun pick(target: Int): Int {
        var n = 0;
        var res = -1;
        for (i in nums.indices) {
            val num = nums[i]
            if (num != target) {
                continue
            }
            n++
            if (Random.nextInt(n) == 0) {
                res = i
            }
        }
        return res
    }

}
