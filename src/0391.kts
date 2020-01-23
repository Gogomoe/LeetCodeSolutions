import kotlin.math.max
import kotlin.math.min
import kotlin.Pair

internal typealias Point = Pair<Int, Int>
internal typealias Rectangle = IntArray

val Rectangle.left: Int
    get() = this[0]
val Rectangle.right: Int
    get() = this[2]
val Rectangle.bottom: Int
    get() = this[1]
val Rectangle.top: Int
    get() = this[3]

class Solution {

    fun isRectangleCover(rectangles: Array<IntArray>): Boolean {
        val first = rectangles[0]
        var left = first.left
        var bottom = first.bottom
        var right = first.right
        var top = first.top

        var area = 0
        val points = mutableSetOf<Point>()
        for (rect in rectangles) {
            left = min(left, rect.left)
            bottom = min(bottom, rect.bottom)
            right = max(right, rect.right)
            top = max(top, rect.top)

            area += (rect.right - rect.left) * (rect.top - rect.bottom)
            val corner = listOf(
                rect.left to rect.top, rect.left to rect.bottom,
                rect.right to rect.top, rect.right to rect.bottom
            )
            for (p in corner) {
                if (p in points) {
                    points.remove(p)
                } else {
                    points.add(p)
                }
            }
        }
        return points.size == 4 && area == (right - left) * (top - bottom) && points.all { (x, y) ->
            (x == left || x == right) && (y == bottom || y == top)
        }
    }
}