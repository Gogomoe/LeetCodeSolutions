import java.util.TreeSet


class Solution {
    fun rectangleArea(rectangles: Array<IntArray>): Int {
        val mod = (1e9 + 7).toInt()
        val xs = TreeSet<Int>()
        val list = mutableListOf<IntArray>()
        for (r in rectangles) {
            xs.add(r[0])
            xs.add(r[2])
            list.add(intArrayOf(r[1], r[0], r[2], 1))
            list.add(intArrayOf(r[3], r[0], r[2], -1))
        }

        var comparator = Comparator<IntArray> { o1, o2 -> o1[0] - o2[0] }
        comparator = comparator.then(Comparator<IntArray> { o1, o2 -> o1[1] - o2[1] })
        comparator = comparator.then(Comparator<IntArray> { o1, o2 -> o1[2] - o2[2] })
        comparator = comparator.then(Comparator<IntArray> { o1, o2 -> o1[3] - o2[3] })

        list.sortWith(comparator)

        val xArr = xs.toIntArray()
        val segment = SegmentTree(xArr)
        val mapIndex = mutableMapOf<Int, Int>()
        for (i in xArr.indices) {
            mapIndex[xArr[i]] = i
        }

        var currentY = 0
        var currentArea = 0
        var result: Long = 0

        for ((y, x1, x2, sign) in list) {
            result += ((y - currentY) * currentArea.toLong()) % mod
            currentY = y
            currentArea = segment.update(mapIndex[x1]!!, mapIndex[x2]!!, sign)
        }

        return (result % mod).toInt()

    }

    class SegmentTree {

        class Node(val start: Int, val end: Int, val cover: Int) {
            var value: Int = 0
            var left: Node? = null
            var right: Node? = null
            var lazy: Int = 0
        }

        val size: Int
        val root: Node

        constructor(xs: IntArray) {
            size = xs.size - 1
            root = buildTree(xs, 0, size)
        }

        private fun buildTree(xs: IntArray, start: Int, end: Int): Node {
            val node = Node(start, end, xs[end] - xs[start])
            if (start + 1 != end) {
                val mid = (start + end) / 2
                node.left = buildTree(xs, start, mid)
                node.right = buildTree(xs, mid, end)
            }
            return node
        }

        fun update(left: Int, right: Int, value: Int): Int {
            return update(left, right, value, root)
        }

        private fun update(left: Int, right: Int, value: Int, node: Node): Int {
            if (left <= node.start && node.end <= right) {
                node.lazy += value
            } else {
                val mid = (node.start + node.end) / 2
                if (left < mid) {
                    update(left, right, value, node.left!!)
                }
                if (right > mid) {
                    update(left, right, value, node.right!!)
                }
            }
            node.value = if (node.lazy > 0) {
                node.cover
            } else {
                (node.left?.value ?: 0) + (node.right?.value ?: 0)
            }
            return node.value
        }

    }
}
