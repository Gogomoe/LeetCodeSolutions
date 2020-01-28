import kotlin.random.Random
import kotlin.Pair

class Solution(rects: Array<IntArray>) {

    private val random = Random.Default

    private val n = rects.size

    private val partSum: List<Pair<Int, IntArray>>
    private val allArea: Int

    init {
        val areas = mutableListOf<Pair<Int, IntArray>>()
        var sum = 0
        for (i in rects.indices) {
            val rect = rects[i]
            val area = (rect[3] - rect[1] + 1) * (rect[2] - rect[0] + 1)
            areas.add(sum to rect)
            sum += area
        }
        allArea = sum
        partSum = areas
    }

    fun pick(): IntArray {
        val it = random.nextInt(allArea)
        var i = 0
        while (i < n && it >= partSum[i].first) {
            i++
        }
        i--
        val remain = it - partSum[i].first
        val rect = partSum[i].second
        val width = rect[2] - rect[0] + 1
        return intArrayOf(
            rect[0] + remain % width,
            rect[1] + remain / width
        )
    }

}
