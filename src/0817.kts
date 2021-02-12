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
    fun numComponents(head: ListNode?, G: IntArray): Int {
        val set = G.toSet()

        var node = head
        var size = set.size

        while (node != null) {
            if (node.`val` in set && (node.next != null && node.next!!.`val` in set)) {
                size--
            }
            node = node.next
        }

        return size
    }
}