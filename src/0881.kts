class Solution {
    fun numRescueBoats(people: IntArray, limit: Int): Int {
        people.sort()
        var L = 0
        var R = people.size - 1
        var count = 0
        while (L < R) {
            if (people[L] + people[R] <= limit) {
                L++
            }
            R--
            count++
        }
        if (L == R) {
            count++
        }
        return count
    }
}