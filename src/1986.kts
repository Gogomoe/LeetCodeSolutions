import kotlin.math.min

class Solution {
    private val memorize = HashMap<String, Int>()

    fun minSessions(tasks: IntArray, sessionTime: Int): Int {
        return helper(0, mutableListOf(), tasks, sessionTime)
    }

    private fun helper(pos: Int, sessions: MutableList<Int>, tasks: IntArray, sessionTime: Int): Int {
        val key = buildKey(pos, sessions)
        if (memorize.containsKey(key)) {
            return memorize[key]!!
        }

        if (pos == tasks.size) {
            return sessions.size
        }

        val time = tasks[pos]
        sessions.add(time)
        var ans = helper(pos + 1, sessions, tasks, sessionTime)
        sessions.removeAt(sessions.size - 1)

        for (i in sessions.indices) {
            if (sessions[i] + time <= sessionTime) {
                sessions[i] += time
                ans = min(ans, helper(pos + 1, sessions, tasks, sessionTime))
                sessions[i] -= time
            }
        }

        memorize[key] = ans
        return ans
    }

    private fun buildKey(pos: Int, sessions: MutableList<Int>): String {
        return "$pos ${sessions.joinToString()}"
    }
}