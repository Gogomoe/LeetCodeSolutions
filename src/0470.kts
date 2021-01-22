/**
 * The rand7() API is already defined in the parent class SolBase.
 * fun rand7(): Int {}
 * @return a random integer in the range 1 to 7
 */
open class SolBase {
    fun rand7(): Int = (Math.random() * 7 + 1).toInt()
}

class Solution : SolBase() {
    fun rand10(): Int {
        var acc = 10.0 / 7.0
        var res = 1 + (rand7() - 1) * acc
        acc /= 7.0
        while (acc > 0.0001) {
            res += (rand7() - 1) * acc
            acc /= 7.0
        }
        return res.toInt()
    }
}