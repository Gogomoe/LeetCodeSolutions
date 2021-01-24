import java.util.*

class MyCalendar() {

    private val tree = TreeMap<Int, Int>()

    fun book(start: Int, end: Int): Boolean {
        val prev = tree.floorEntry(start)?.value ?: Int.MIN_VALUE < start
        val next = tree.ceilingEntry(start + 1)?.key ?: Int.MAX_VALUE > end - 1
        if (prev && next) {
            tree[start] = end - 1
            return true
        }
        return false
    }

}

/**
 * Your MyCalendar object will be instantiated and called as such:
 * var obj = MyCalendar()
 * var param_1 = obj.book(start,end)
 */