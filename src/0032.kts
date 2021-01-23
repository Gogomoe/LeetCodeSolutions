class Solution {
    fun longestValidParentheses(s: String): Int {
        var resBegin = 1
        var resEnd = 0

        var stack = 0

        var begin = 0
        var end = 0
        for (i in s.indices) {
            when (s[i]) {
                '(' -> stack++
                ')' -> stack--
            }
            if (stack == 0) {
                end = i
                if ((end - begin) > (resEnd - resBegin)) {
                    resEnd = end
                    resBegin = begin
                }
            } else if (stack < 0) {
                stack = 0
                begin = i + 1
            }
        }

        stack = 0
        end = s.length - 1
        for (i in s.indices.reversed()) {
            when (s[i]) {
                '(' -> stack--
                ')' -> stack++
            }
            if (stack == 0) {
                begin = i
                if ((end - begin) > (resEnd - resBegin)) {
                    resEnd = end
                    resBegin = begin
                }
            } else if (stack < 0) {
                stack = 0
                end = i - 1
            }
        }

        return (resEnd - resBegin + 1)
    }
}