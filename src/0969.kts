class Solution {
    fun pancakeSort(A: IntArray): List<Int> {
        val result = mutableListOf<Int>()
        for (i in A.indices.reversed()) {
            val index = findIndexOfMax(A, 0..i)
            pancakeFlip(A, index)
            result.add(index + 1)
            pancakeFlip(A, i)
            result.add(i + 1)
        }
        return result
    }

    private fun pancakeFlip(arr: IntArray, index: Int) {
        var left = 0
        var right = index
        while (left < right) {
            swap(arr, left, right)
            left++
            right--
        }
    }

    private fun swap(arr: IntArray, left: Int, right: Int) {
        arr[left] = arr[right].also { arr[right] = arr[left] }
    }

    private fun findIndexOfMax(arr: IntArray, range: IntRange): Int {
        var index = range.first
        var value = arr[range.first]
        for (i in range) {
            if (arr[i] > value) {
                index = i
                value = arr[i]
            }
        }
        return index
    }
}