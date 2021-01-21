class Solution {
    fun findSubsequences(nums: IntArray): List<List<Int>> {
        if (nums.size < 2) {
            return emptyList()
        }
        val result = mutableSetOf<List<Int>>()
        dfs(nums, result, emptyList(), 0)
        return result.toList()
    }

    private fun dfs(nums: IntArray, result: MutableSet<List<Int>>, answer: List<Int>, index: Int) {
        if (index == nums.size) {
            return
        }
        dfs(nums, result, answer, index + 1)
        if (answer.isEmpty() || nums[index] >= answer.last()) {
            val newList = answer.toMutableList().apply { add(nums[index]) }
            if (newList.size >= 2) {
                result.add(newList)
            }
            dfs(nums, result, newList, index + 1)
        }
    }
}