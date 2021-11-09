class Solution {
    fun permuteUnique(nums: IntArray): List<List<Int>> {
        val counts = nums.asSequence().groupingBy { it }.eachCount().toMutableMap()
        val result = mutableListOf<List<Int>>()
        dfs(mutableListOf<Int>(), counts, result, nums.size)
        return result
    }

    private fun dfs(list: MutableList<Int>, counts: MutableMap<Int, Int>, result: MutableList<List<Int>>, target: Int) {
        if (list.size == target) {
            result.add(list.toList())
            return
        }
        for ((k, v) in counts) {
            if (v != 0) {
                list.add(k)
                counts[k] = v - 1
                dfs(list, counts, result, target)
                counts[k] = v
                list.removeAt(list.size - 1)
            }
        }
    }
}

println(Solution().permuteUnique(intArrayOf(1, 1, 2))) // [[1, 1, 2], [1, 2, 1], [2, 1, 1]]
println(Solution().permuteUnique(intArrayOf(1, 2, 3))) // [[1, 2, 3], [1, 3, 2], [2, 1, 3], [2, 3, 1], [3, 1, 2], [3, 2, 1]]