class ListNode(var `val`: Int) {
    var next: ListNode? = null
}

class Solution {
    fun addTwoNumbers(l1: ListNode?, l2: ListNode?): ListNode? {
        var a: ListNode? = l1
        var b: ListNode? = l2
        val header = ListNode(-1)
        var last: ListNode = header
        var carry = 0
        while (a != null || b != null || carry == 1) {
            val sum = carry + when {
                a == null && b == null -> 0
                a == null -> b!!.`val`
                b == null -> a.`val`
                else -> a.`val` + b.`val`
            }
            val next = ListNode(sum % 10)
            last.next = next
            last = next

            carry = sum / 10

            a = a?.next
            b = b?.next
        }
        return header.next
    }
}