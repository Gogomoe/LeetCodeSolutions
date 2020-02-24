import java.util.*

class Solution {
    fun calculate(s: String): Int {
        val deque = ArrayDeque(s.toList())
        return exp(deque);
    }

    private fun exp(deque: ArrayDeque<Char>): Int {
        var left = factor(deque)
        var operator = operator(deque)
        while (operator != null) {
            val right = factor(deque)
            left = calc(left, right, operator)
            operator = operator(deque)
        }
        return left
    }

    private fun calc(left: Int, right: Int, operator: Char): Int {
        return when (operator) {
            '+' -> left + right
            '-' -> left - right
            else -> throw IllegalStateException()
        }
    }

    private fun operator(deque: ArrayDeque<Char>): Char? {
        while (deque.isNotEmpty() && deque.first == ' ') {
            deque.removeFirst()
        }
        if (deque.isEmpty()) {
            return null
        }
        return when (deque.first) {
            '+', '-' -> deque.removeFirst()
            else -> null
        }
    }

    private fun factor(deque: ArrayDeque<Char>): Int {
        while (deque.first == ' ') {
            deque.removeFirst()
        }
        return when (deque.first) {
            '(' -> {
                deque.removeFirst()
                val res = exp(deque)
                while (deque.first == ' ') {
                    deque.removeFirst()
                }
                deque.removeFirst()
                res
            }
            else -> {
                var num = deque.removeFirst() - '0'
                while (deque.isNotEmpty() && deque.first in '0'..'9') {
                    num = num * 10 + (deque.removeFirst() - '0')
                }
                num
            }
        }
    }

}