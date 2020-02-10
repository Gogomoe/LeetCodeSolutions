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
    fun reverseKGroup(head: ListNode?, k: Int): ListNode? {
        if (k == 1) {
            return head
        }

        val first = ListNode(-1)
        var lastEnd = first
        var header = head
        var end = findEnd(header, k)
        var nextHeader = end?.next
        while (end != null) {
            var last = header!!
            var current = header.next!!
            while (current != end) {
                val next = current.next!!
                current.next = last
                last = current
                current = next
            }
            current.next = last
            lastEnd.next = end

            lastEnd = header
            header = nextHeader
            end = findEnd(header, k)
            nextHeader = end?.next
        }
        lastEnd.next = header
        return first.next
    }

    private fun findEnd(head: ListNode?, k: Int): ListNode? {
        var current = head ?: return null
        repeat(k - 1) {
            current = current.next ?: return null
        }
        return current
    }
}
