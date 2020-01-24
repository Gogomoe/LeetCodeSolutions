import java.lang.StringBuilder
import kotlin.math.min

class Solution {
    fun isRationalEqual(S: String, T: String): Boolean {
        val (sInt, sNonRepeat, sRepeat) = parse(S)
        val (tInt, tNonRepeat, tRepeat) = parse(T)

        if (sInt != tInt) {
            return false
        }

        val s = StringBuilder()
        val t = StringBuilder()

        s.append(sNonRepeat)
        t.append(tNonRepeat)

        while (s.length < tNonRepeat.length) {
            s.append(sRepeat)
        }

        while (t.length < sNonRepeat.length) {
            t.append(tRepeat)
        }

        s.append(sRepeat)
        t.append(tRepeat)

        for (i in 0 until min(s.length, t.length)) {
            if (s[i] != t[i]) {
                return false
            }
        }
        return true
    }

    private fun parse(s: String): Triple<String, String, String> {
        val dot = s.indexOf('.')
        val bracket = s.indexOf('(')
        return when {
            dot == -1 -> Triple(s, "", "0")
            bracket == -1 -> Triple(s.substring(0, dot), s.substring(dot + 1), "0")
            else -> Triple(s.substring(0, dot), s.substring(dot + 1, bracket), s.substring(bracket + 1, s.length - 1))
        }.let(::handleRepeatingNine)
    }

    private fun handleRepeatingNine(str: Triple<String, String, String>): Triple<String, String, String> {
        val (Int, Non, Rep) = str
        if (!Rep.all { it == '9' }) {
            return str
        }

        val len = Non.length
        var num = Int.toInt()
        repeat(len) {
            num *= 10
        }
        num += if (Non.isEmpty()) 0 else Non.toInt()
        num += 1

        val res = num.toString().padStart(Int.length + Non.length, '0')
        val endIndex = res.length - len
        return Triple(res.substring(0, endIndex), res.substring(endIndex), "0")
    }
}