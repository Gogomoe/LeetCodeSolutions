class Solution {
    fun numOfSubarrays(arr: IntArray): Int {
        val odds = IntArray(arr.size)
        val evens = IntArray(arr.size)

        odds[0] += arr[0] % 2
        evens[0] += (arr[0] + 1) % 2

        for (i in 1 until arr.size) {
            if (arr[i] % 2 == 0) {
                odds[i] = odds[i - 1]
                evens[i] = evens[i - 1] + 1
            } else {
                odds[i] = evens[i - 1] + 1
                evens[i] = odds[i - 1]
            }
        }

        return odds.fold(0L, Long::plus).rem(1000000007).toInt()
    }
}