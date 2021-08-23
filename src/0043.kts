class Solution {
    fun multiply(num1: String, num2: String): String {
        if (num1 == "0" || num2 == "0") {
            return "0"
        }
        val result = IntArray(num1.length + num2.length)
        for (i in num1.indices) {
            for (j in num2.indices) {
                val mul = (num1[i] - '0') * (num2[j] - '0')
                result[i + j + 1] += mul
            }
        }
        val builder = StringBuilder()
        for (i in (result.size - 1) downTo 1) {
            result[i - 1] += result[i] / 10
            builder.append((result[i] % 10 + '0'.code).toChar())
        }
        if (result[0] != 0) {
            builder.append((result[0] + '0'.code).toChar())
        }
        return builder.reverse().toString()
    }
}
