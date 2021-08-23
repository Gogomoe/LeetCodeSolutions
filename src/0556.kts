class Solution {
    fun nextGreaterElement(n: Int): Int {
        var num = n
        val digits = mutableListOf<Int>()
        while (num != 0) {
            digits.add(num % 10)
            num /= 10
        }
        var p = 0
        while (p < digits.size - 1 && digits[p] <= digits[p + 1]) {
            p++
        }
        if (p == digits.size - 1) return -1

        val pa = p + 1
        val a = digits[pa]
        val frontDigits = digits.subList(0, pa)
        val b = frontDigits.filter { it > a }.minOrNull()!!
        val pb = frontDigits.lastIndexOf(b)

        frontDigits[pb] = a
        frontDigits.sortDescending()

        for (i in frontDigits.indices) {
            digits[i] = frontDigits[i]
        }
        digits[pa] = b

        return digits.foldRight(0L) { it, acc ->
            acc * 10 + it
        }.let { if (it < Int.MAX_VALUE) it.toInt() else -1 }
    }
}

println(Solution().nextGreaterElement(1999999999))