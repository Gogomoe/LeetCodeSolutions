class Solution {
    fun largestValsFromLabels(values: IntArray, labels: IntArray, num_wanted: Int, use_limit: Int): Int {
        val zip = values.zip(labels).sortedByDescending { it.first }
        var selected = 0
        val used = mutableMapOf<Int, Int>().withDefault { 0 }
        var sum = 0
        for ((value, label) in zip) {
            val usedCount = used.getValue(label)
            if (usedCount < use_limit) {
                used[label] = usedCount + 1
                selected++
                sum += value
                if (selected == num_wanted) {
                    break
                }
            }
        }
        return sum
    }
}