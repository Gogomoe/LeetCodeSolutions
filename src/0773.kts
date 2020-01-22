import java.util.*
import kotlin.Pair

class Solution {

    fun slidingPuzzle(board: Array<IntArray>): Int {
        val used = mutableSetOf<Int>()
        val queue: Queue<Pair<IntArray, Int>> = ArrayDeque()

        val start = intArrayOf(*board[0], *board[1])
        queue.add(start to 0)
        var hash = hash(start)
        used.add(hash)
        if (hash == SOLVED) {
            return 0
        }

        while (queue.isNotEmpty()) {
            val (b, c) = queue.poll()
            for (next in nextBoard(b)) {
                hash = hash(next)
                if (hash !in used) {
                    queue.add(next to c + 1)
                    used.add(hash)
                    if (hash == SOLVED) {
                        return c + 1
                    }
                }
            }
        }
        return -1
    }

    private fun nextBoard(b: IntArray): List<IntArray> {
        val res = mutableListOf<IntArray>()
        val i = b.indexOf(0)
        if (i != 0 && i != 3) {
            res.add(b.clone().also { it.swap(i, i - 1) })
        }
        if (i != 2 && i != 5) {
            res.add(b.clone().also { it.swap(i, i + 1) })
        }
        if (i <= 2) {
            res.add(b.clone().also { it.swap(i, i + 3) })
        }
        if (i >= 3) {
            res.add(b.clone().also { it.swap(i, i - 3) })
        }
        return res
    }

    private val SOLVED = 1 * 1 + 2 * 6 + 3 * 6 * 6 + 4 * 6 * 6 * 6 + 5 * 6 * 6 * 6 * 6
    private fun hash(arr: IntArray): Int {
        var pow = 1
        var i = 0
        var sum = 0
        while (i < 6) {
            sum += arr[i] * pow
            pow *= 6
            i += 1
        }
        return sum
    }

    private fun IntArray.swap(a: Int, b: Int) {
        this[a] = this[b].also { this[b] = this[a] }
    }

}