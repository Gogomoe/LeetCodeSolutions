import java.lang.IllegalStateException

class Solution {
    fun romanToInt(s: String): Int {
        val c = s.toCharArray()
        var i = 0
        var res = 0
        while (i < c.size) {
            res += when (c[i]) {
                'I' -> {
                    when (c.getOrNull(i + 1)) {
                        'V' -> {
                            i++
                            4
                        }
                        'X' -> {
                            i++
                            9
                        }
                        else -> 1
                    }
                }
                'V' -> {
                    5
                }
                'X' -> {
                    when (c.getOrNull(i + 1)) {
                        'L' -> {
                            i++
                            40
                        }
                        'C' -> {
                            i++
                            90
                        }
                        else -> 10
                    }
                }
                'L' -> {
                    50
                }
                'C' -> {
                    when (c.getOrNull(i + 1)) {
                        'D' -> {
                            i++
                            400
                        }
                        'M' -> {
                            i++
                            900
                        }
                        else -> 100
                    }
                }
                'D' -> 500
                'M' -> 1000
                else -> throw IllegalStateException()
            }
            i++
        }
        return res
    }
}

println(Solution().romanToInt("MCMXCIV"))