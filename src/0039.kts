class Solution {
    fun combinationSum(candidates: IntArray, target: Int): List<List<Int>> {
        candidates.sort()
        val result = mutableListOf<List<Int>>()
        val list = mutableListOf<Int>()
        dfs(0, 0, list, candidates, target, result)
        return result
    }

    private fun dfs(
        pos: Int,
        sum: Int,
        list: MutableList<Int>,
        candidates: IntArray,
        target: Int,
        result: MutableList<List<Int>>
    ) {
        if (pos == candidates.size) {
            return
        }
        if (sum == target) {
            result.add(list.toList())
        }
        for (i in pos until candidates.size) {
            val it = candidates[i]
            if (sum + it > target) {
                break
            }
            list.add(it)
            dfs(i, sum + it, list, candidates, target, result)
            list.removeAt(list.size - 1)
        }
    }
}

println(Solution().combinationSum(intArrayOf(2, 3, 6, 7), 7)) // [[2,2,3],[7]]
println(Solution().combinationSum(intArrayOf(2, 3, 5), 8)) // [[2,2,2,2],[2,3,3],[3,5]]
println(Solution().combinationSum(intArrayOf(2), 1)) // []
println(Solution().combinationSum(intArrayOf(1), 1)) // [[1]]
println(Solution().combinationSum(intArrayOf(1), 2)) // [[1,1]]