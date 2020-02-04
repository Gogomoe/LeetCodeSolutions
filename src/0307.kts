import kotlin.math.min

class NumArray(val nums: IntArray) {

    val partSum = IntArray(nums.size)
    var dirtyFrom = nums.size

    init {
        var sum = 0
        for (i in nums.indices) {
            sum += nums[i]
            partSum[i] = sum
        }
    }

    fun update(i: Int, `val`: Int) {
        nums[i] = `val`
        dirtyFrom = min(dirtyFrom, i)
    }

    fun sumRange(i: Int, j: Int): Int {
        if (dirtyFrom <= j) {
            var sum = if (dirtyFrom == 0) 0 else partSum[dirtyFrom - 1]
            for (k in dirtyFrom..j) {
                sum += nums[k]
                partSum[k] = sum
            }
            dirtyFrom = j + 1
        }
        return partSum[j] - if (i == 0) 0 else partSum[i - 1]
    }

}