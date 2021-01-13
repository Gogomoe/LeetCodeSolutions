class Solution {
    fun minSteps(n: Int): Int {
        var sum = 0
        var it = n
        var i = 2
        while (it != 1) {
            if (it % i == 0) {
                sum += i
                it /= i
            } else {
                i += 1
            }
        }
        return sum
    }
}