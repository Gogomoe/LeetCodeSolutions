import java.util.*

/**
 * Example:
 * var li = ListNode(5)
 * var v = li.`val`
 * Definition for singly-linked list.
 * class ListNode(var `val`: Int) {
 *     var next: ListNode? = null
 * }
 */

class ListNode(var `val`: Int) {
    var next: ListNode? = null
}

class Solution {
    fun mergeKLists(lists: Array<ListNode?>): ListNode? {
        if (lists.isEmpty() || lists.all { it == null }) {
            return null
        }
        val head = ListNode(-1)
        var last = head

        val queue = PriorityQueue<ListNode> { a, b ->
            a.`val` - b.`val`
        }
        lists.filterNotNull().forEach {
            queue.add(it)
        }
        while (queue.isNotEmpty()) {
            val it = queue.poll()
            last.next = it
            last = it

            if (it!!.next != null) {
                queue.add(it.next)
            }
        }
        return head.next
    }
}