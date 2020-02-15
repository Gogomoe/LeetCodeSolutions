class Solution {
    fun fizzBuzz(n: Int): List<String> {
        val res = mutableListOf<String>()
        for (i in 1..n) {
            res.add(
                when {
                    i % 15 == 0 -> "FizzBuzz"
                    i % 5 == 0 -> "Buzz"
                    i % 3 == 0 -> "Fizz"
                    else -> i.toString()
                }
            )
        }
        return res
    }
}