class Solution {
    fun rotatedDigits(N: Int): Int {
        var count = 0
        loop@ for (i in 1..N) {
            var x = i

            var isDifferent = false
            while (x != 0) {
                var d = x % 10
                when (d) {
                    2, 5, 6, 9 -> {
                        isDifferent = true
                    }
                    0, 1, 8 -> {
                    }
                    else -> {
                        continue@loop
                    }
                }
                x /= 10
            }
            if (isDifferent) {
                count++
            }
        }
        return count
    }
}