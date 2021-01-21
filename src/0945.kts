class Solution {
    fun minIncrementForUnique(A: IntArray): Int {
        A.sort()
        var need = 0
        var res = 0
        for (it in A) {
            if (it < need) {
                res += need - it
            }
            need = maxOf(need, it) + 1
        }
        return res
    }
}