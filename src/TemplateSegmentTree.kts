class SegmentTree {

    class Node(val start: Int, val end: Int, var value: Int = 0) {
        var left: Node? = null
        var right: Node? = null
        var lazy: Int = 0
    }

    val size: Int
    val root: Node

    constructor(n: Int) {
        size = n
        root = buildTree(0, n)
    }

    private fun buildTree(start: Int, end: Int): Node {
        val node = Node(start, end)
        if (start + 1 != end) {
            val mid = (start + end) / 2
            node.left = buildTree(start, mid)
            node.right = buildTree(mid, end)
        }
        return node
    }

    fun sum(left: Int, right: Int): Int {
        return sum(left, right, root)
    }

    private fun sum(left: Int, right: Int, node: Node): Int {
        if (left <= node.start && node.end <= right) {
            return node.value
        }
        val mid = (left + right) / 2
        pushDownLazyValue(node)

        var result = 0
        if (left < mid) {
            result += sum(left, right, node.left!!)
        }
        if (right >= mid) {
            result += sum(left, right, node.right!!)
        }
        return result
    }

    fun update(left: Int, right: Int, value: Int) {
        update(left, right, value, root)
    }

    private fun update(left: Int, right: Int, value: Int, node: Node) {
        if (left <= node.start && node.end <= right) {
            node.value += (node.end - node.start) * value
            node.lazy += value
            return
        }
        val mid = (left + right) / 2
        pushDownLazyValue(node)

        if (left < mid) {
            update(left, right, value, node.left!!)
        }
        if (right >= mid) {
            update(left, right, value, node.right!!)
        }
        node.value = node.left!!.value + node.right!!.value
    }

    private fun pushDownLazyValue(node: Node) {
        if (node.lazy != 0 && node.start + 1 != node.end) {
            val mid = (node.start + node.end) / 2
            node.left!!.value += node.lazy * (mid - node.start)
            node.left!!.lazy += node.lazy
            node.right!!.value += node.lazy * (node.end - mid)
            node.right!!.lazy += node.lazy
            node.lazy = 0
        }
    }

}