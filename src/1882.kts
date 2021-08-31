import java.util.*
import kotlin.math.max

data class Server(
    val weight: Int,
    val index: Int
) {
    companion object {
//        val COMPARATOR: Comparator<Server> = Comparator
//            .thenComparingInt { it.weight }
//            .thenComparingInt { it.index }

        val COMPARATOR: Comparator<Server> = Comparator { o1: Server, o2: Server ->
            when {
                o1.weight != o2.weight -> o1.weight - o2.weight
                else -> o1.index - o2.index
            }
        }
    }
}

class Solution {
    fun assignTasks(servers: IntArray, tasks: IntArray): IntArray {
        val available = TreeSet<Server>(Server.COMPARATOR)
        servers.forEachIndexed { index, weight ->
            available.add(Server(weight, index))
        }
        val work = TreeMap<Int, MutableList<Server>>()
        var currentTime = 0

        return tasks.mapIndexed { index, timeNeed ->
            currentTime = max(
                currentTime,
                if (available.isEmpty()) max(index, work.firstKey()) else index
            )

            while (work.isNotEmpty() && work.firstKey() <= currentTime) {
                available.addAll(work.firstEntry().value)
                work.remove(work.firstKey())
            }

            val server = available.first()
            available.remove(server)

            val finishTime = currentTime + timeNeed
            work.putIfAbsent(finishTime, mutableListOf())
            work[finishTime]!!.add(server)

            server.index

        }.toIntArray()
    }
}

println(
    Solution().assignTasks(
        intArrayOf(3, 3, 2),
        intArrayOf(1, 2, 3, 2, 1, 2)
    ).contentToString()
)

println(
    Solution().assignTasks(
        intArrayOf(10, 63, 95, 16, 85, 57, 83, 95, 6, 29, 71),
        intArrayOf(70, 31, 83, 15, 32, 67, 98, 65, 56, 48, 38, 90, 5)
    ).contentToString()
)
