class Solution {
    fun isMatch(s: String, p: String): Boolean {
        val arr = Array(s.length + 1) { BooleanArray(p.length + 1) }
        arr[0][0] = true
        for (i in 1 until p.length step 2) {
            if (p[i] == '*' && arr[0][i - 1]) {
                arr[0][i + 1] = true
            } else {
                break
            }
        }

        for (i in s.indices) {
            for (j in p.indices) {
                if (p[j] == s[i]) {
                    arr[i + 1][j + 1] = arr[i][j]
                } else if (p[j] == '.') {
                    arr[i + 1][j + 1] = arr[i][j]
                } else if (p[j] == '*') {
                    if (p[j - 1] == s[i] || p[j - 1] == '.') {
                        arr[i + 1][j + 1] = arr[i][j + 1] || arr[i + 1][j] || arr[i + 1][j - 1]
                    } else {
                        arr[i + 1][j + 1] = arr[i + 1][j - 1]
                    }
                }
            }
        }

        return arr[s.length][p.length]
    }
}