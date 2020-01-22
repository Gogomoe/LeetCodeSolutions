class Solution {
    fun hasAlternatingBits(n: Int): Boolean {
        var bit = n % 2
        var x = n / 2
        while (x != 0) {
            val newBit = x % 2
            if (newBit == bit) {
                return false
            }
            bit = newBit
            x /= 2
        }
        return true
    }
}