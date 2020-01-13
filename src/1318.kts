class Solution {
    fun minFlips(a: Int, b: Int, c: Int): Int {
        val max = 1 shl 31

        var mask = 1
        var count = 0
        while (mask != max) {
            val pa = a and mask
            val pb = b and mask
            val pc = c and mask
            if (pa == 0 && pb == 0 && pc != 0) {
                count++
            } else if (pc == 0 && pa or pb != 0 && pa xor pb == 0) {
                count += 2
            } else if (pc == 0 && pa or pb != 0 && pa xor pb != 0) {
                count++
            }
            mask = mask shl 1
        }
        return count
    }
}