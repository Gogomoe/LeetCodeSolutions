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
    fun oddEvenList(head: ListNode?): ListNode? {
        val evenHeader = ListNode(-1)
        var evenTail = evenHeader

        val oddHeader = ListNode(-2)
        var oddTail = oddHeader

        var node = head
        var isEven = true
        while (node != null){
            if (isEven){
                evenTail.next = node
                evenTail = node
            } else{
                oddTail.next = node
                oddTail = node
            }
            isEven = !isEven
            val next = node.next
            node.next = null
            node = next
        }
        evenTail.next = oddHeader.next
        return evenHeader.next
    }
}