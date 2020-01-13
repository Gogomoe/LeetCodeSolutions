import java.util.*
import kotlin.Comparator

class Node(
    val key: Int,
    val values: List<String>,
    var friends: IntArray = IntArray(0)
)

class Solution {
    fun watchedVideosByFriends(
        watchedVideos: List<List<String>>,
        friends: Array<IntArray>,
        id: Int,
        level: Int
    ): List<String> {
        val n = friends.size
        val nodes = Array(n) { Node(it, watchedVideos[it], friends[it]) }
        val used: BooleanArray = BooleanArray(n)

        val queue: Queue<Pair<Int, Node>> = ArrayDeque()
        queue.add(0 to nodes[id])
        used[id] = true
        while (queue.first().first != level) {
            val (l, node) = queue.poll()
            for (friend in node.friends) {
                if (!used[friend]) {
                    queue.add(l + 1 to nodes[friend])
                    used[friend] = true
                }
            }
        }

        val videos = mutableMapOf<String, Int>()
        while (queue.isNotEmpty()) {
            val (_, node) = queue.poll()
            for (value in node.values) {
                videos.merge(value, 1) { a, b ->
                    a + b
                }
            }
        }

        return videos.entries.sortedWith(Comparator { o1, o2 ->
            if (o1.value == o2.value) o1.key.compareTo(o2.key) else o1.value - o2.value
        }).map { it.key }.toList()
    }
}