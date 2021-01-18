class RLEIterator(private val arr: IntArray) {

    private var index = 0
    private var count = 0

    fun next(n: Int): Int {
        var it = n
        while (it != 0 && index != arr.size) {
            val size = arr[index]
            val consume = minOf(size - count, it)

            count += consume
            it -= consume

            if (count == size) {
                index += 2
                count = 0
            }
        }
        return when {
            index == arr.size && it != 0 -> -1
            count == 0 -> arr[index - 1]
            else -> arr[index + 1]
        }
    }

}

/**
 * Your RLEIterator object will be instantiated and called as such:
 * var obj = RLEIterator(A)
 * var param_1 = obj.next(n)
 */