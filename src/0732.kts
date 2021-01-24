import java.util.*

class MyCalendarThree() {

    private val tree = TreeMap<Int, Int>()
    private var res = 0

    init {
        tree[-1] = 0
    }

    fun book(start: Int, end: Int): Int {
        tree[start] = tree.floorEntry(start)?.value ?: 0
        tree[end] = tree.floorEntry(end)?.value ?: 0
        for ((k, v) in tree.subMap(start, end).entries) {
            tree[k] = v + 1
            res = maxOf(res, v + 1)
        }
        return res
    }

}

/**
 * Your MyCalendarThree object will be instantiated and called as such:
 * var obj = MyCalendarThree()
 * var param_1 = obj.book(start,end)
 */