class Solution {
    fun largestDivisibleSubset(nums: IntArray): List<Int> {
        val res = mutableMapOf<Int, Set<Int>>()
        for (i in nums.sorted()) {
            res[i] = (res.filterKeys { i % it == 0 }.values.maxByOrNull { it.size }?.toMutableSet()
                ?: mutableSetOf()).also { it.add(i) }
        }
        return res.values.maxByOrNull { it.size }?.toList() ?: emptyList()
    }
}