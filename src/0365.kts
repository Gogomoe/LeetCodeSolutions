class Solution {
    fun canMeasureWater(x: Int, y: Int, z: Int): Boolean {
        if (x + y < z) {
            return false
        }
        if ((x == 0 || y == 0) && x + y == z) {
            return true
        }
        return z % GCD(x, y) == 0
    }

    fun GCD(x: Int, y: Int): Int {
        var a = x
        var b = y
        while (b != 0) {
            val tmp = b
            b = a % b
            a = tmp
        }
        return a
    }
}