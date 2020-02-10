class Solution {
    fun plusOne(digits: IntArray): IntArray {
        var add = 1
        val res = IntArray(digits.size)
        for (i in digits.indices.reversed()) {
            res[i] = (digits[i] + add) % 10
            add = if (add == 1 && res[i] == 0) 1 else 0
        }
        if (add == 1) {
            return IntArray(digits.size + 1).also {
                it[0] = 1
            }
        }
        return res
    }
}