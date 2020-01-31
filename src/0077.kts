class Solution {
    fun combine(n: Int, k: Int): List<List<Int>> {
        fun dfs(pos: Int, start: Int, list: MutableList<List<Int>>, intArray: IntArray) {
            if (start > n) {
                return
            }
            for (i in start..n) {
                intArray[pos] = i
                if (pos == k - 1) {
                    list.add(intArray.toList())
                } else {
                    dfs(pos + 1, i + 1, list, intArray)
                }
            }

        }

        val list = mutableListOf<List<Int>>()
        dfs(0, 1, list, IntArray(k))
        return list
    }

}