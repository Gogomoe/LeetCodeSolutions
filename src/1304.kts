class Solution {
    fun sumZero(n: Int): IntArray {
        val arr = IntArray(n)
        var it = 0
        for (i in -n / 2..n / 2) {
            if (i != 0 || n % 2 == 1){
                arr[it++] = i
            }
        }
        return arr
    }
}