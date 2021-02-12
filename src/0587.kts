import java.util.Comparator

class Solution {
    fun outerTrees(points: Array<IntArray>): Array<IntArray> {
        val result = mutableSetOf<IntArray>()

        points.sortWith(Comparator { (x1, y1), (x2, y2) ->
            if (x1 == x2) {
                y1 - y2
            } else {
                x1 - x2
            }
        })

        val stack = mutableListOf<IntArray>()
        for (it in points) {
            addPoint(stack, it)
        }

        result.addAll(stack)
        stack.clear()

        points.reverse()
        for (it in points) {
            addPoint(stack, it)
        }

        result.addAll(stack)
        return result.toTypedArray()
    }

    private fun addPoint(stack: MutableList<IntArray>, it: IntArray) {
        if (stack.size < 2) {
            stack.add(it)
            return
        }

        while (stack.size >= 2) {
            val p1 = stack[stack.size - 2]
            val p2 = stack[stack.size - 1]
            val p3 = it

            val x1 = p2[0] - p1[0]
            val y1 = p2[1] - p1[1]
            val x2 = p3[0] - p2[0]
            val y2 = p3[1] - p2[1]

            if (x1 * y2 - x2 * y1 < 0) {
                stack.removeAt(stack.size - 1)
            } else {
                break
            }

        }
        stack.add(it)
    }
}