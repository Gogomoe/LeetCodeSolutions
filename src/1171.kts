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
    fun removeZeroSumSublists(head: ListNode?): ListNode? {
        val map = mutableMapOf<Int, ListNode>()
        val header = ListNode(-1)
        header.next = head

        map[0] = header
        var sum = 0
        var node = head
        while (node != null) {
            sum += node.`val`
            if (map[sum] != null) {
                val last = map[sum]!!
                var nodeFromLast = last.next!!
                var sumFromLast = sum
                while (nodeFromLast != node) {
                    sumFromLast += nodeFromLast.`val`
                    map.remove(sumFromLast)
                    nodeFromLast = nodeFromLast.next!!
                }
                last.next = node.next
            } else {
                map[sum] = node
            }
            node = node.next
        }

        return header.next
    }
}