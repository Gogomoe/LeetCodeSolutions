class Solution {
    var result = mutableListOf<String>()

    fun removeInvalidParentheses(s: String): List<String> {
        result = mutableListOf()
        removeRightParentheses(s, 0, 0)
        return result
    }

    private fun removeRightParentheses(s: String, lastRemovePos: Int, visitedPos: Int) {
        var leftParentheses = 0
        for (i in (visitedPos until s.length)) {
            if (s[i] == '(') {
                leftParentheses++
            } else if (s[i] == ')') {
                leftParentheses--
            }
            if (leftParentheses == -1) {
                if (s[lastRemovePos] == ')') {
                    removeRightParentheses(
                            s.substring(0 until lastRemovePos) + s.substring(lastRemovePos + 1 until s.length),
                            lastRemovePos,
                            i
                    )
                }
                for (j in (lastRemovePos + 1)..i) {
                    if (s[j] == ')' && s[j - 1] != ')') {
                        removeRightParentheses(
                                s.substring(0 until j) + s.substring(j + 1 until s.length),
                                j,
                                i
                        )
                    }
                }
                return
            }
        }
        removeLeftParentheses(s.reversed(), 0, 0)
    }

    private fun removeLeftParentheses(s: String, lastRemovePos: Int, visitedPos: Int) {
        var rightParentheses = 0
        for (i in (visitedPos until s.length)) {
            if (s[i] == ')') {
                rightParentheses++
            } else if (s[i] == '(') {
                rightParentheses--
            }
            if (rightParentheses == -1) {
                if (s[lastRemovePos] == '(') {
                    removeLeftParentheses(
                            s.substring(0 until lastRemovePos) + s.substring(lastRemovePos + 1 until s.length),
                            lastRemovePos,
                            i
                    )
                }
                for (j in (lastRemovePos + 1)..i) {
                    if (s[j] == '(' && s[j - 1] != '(') {
                        removeLeftParentheses(
                                s.substring(0 until j) + s.substring(j + 1 until s.length),
                                j,
                                i
                        )
                    }
                }
                return
            }
        }
        result.add(s.reversed())
    }
}
