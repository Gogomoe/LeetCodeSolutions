class Solution {
    fun findSmallestSetOfVertices(n: Int, edges: List<List<Int>>): List<Int> {
        val haveInEdge = BooleanArray(n)
        for ((_, v) in edges) {
            haveInEdge[v] = true
        }
        val result = mutableListOf<Int>()
        for (i in 0 until n) {
            if (!haveInEdge[i]) {
                result.add(i)
            }
        }
        return result
    }
}