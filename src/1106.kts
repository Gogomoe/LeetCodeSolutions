import java.lang.IllegalStateException
import java.util.*

class Solution {

    private lateinit var queue: Queue<Char>
    fun parseBoolExpr(expression: String): Boolean {
        queue = ArrayDeque(expression.toMutableList())
        return expression()
    }

    private fun expression(): Boolean {
        return when (queue.first()) {
            '!' -> {
                queue.poll()
                queue.poll()
                val res = expression()
                queue.poll()
                !res
            }
            '&' -> {
                queue.poll()
                queue.poll()
                val res: List<Boolean> = expressionList()
                queue.poll()
                res.all { it }
            }
            '|' -> {
                queue.poll()
                queue.poll()
                val res: List<Boolean> = expressionList()
                queue.poll()
                res.any { it }
            }
            else -> factor()
        }
    }

    private fun expressionList(): List<Boolean> {
        val list = mutableListOf<Boolean>()
        list.add(expression())
        while (queue.first() == ',') {
            queue.poll()
            list.add(expression())
        }
        return list
    }

    private fun factor(): Boolean {
        return when (queue.poll()) {
            't' -> true
            'f' -> false
            else -> throw IllegalStateException()
        }
    }

}