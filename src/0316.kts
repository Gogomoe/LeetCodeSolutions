class Solution {
    fun removeDuplicateLetters(str: String): String {
        val map = str.asSequence().groupingBy { it }.eachCount().toMutableMap()
        val visit = mutableSetOf<Char>()
        val deque = ArrayDeque<Char>()
        for (c in str) {
            map.compute(c) { _, v -> v!! - 1 }
            if (visit.contains(c)) {
                continue
            }
            while (deque.isNotEmpty() && deque.last() > c && map[deque.last()] != 0) {
                val remove = deque.removeLast()
                visit.remove(remove)
            }
            deque.addLast(c)
            visit.add(c)
        }
        return deque.joinToString("");
    }
}

println(Solution().removeDuplicateLetters("bcabc")) // abc
println(Solution().removeDuplicateLetters("cbacdcbc")) // acdb
println(Solution().removeDuplicateLetters("abfbadcd")) // abfcd
println(Solution().removeDuplicateLetters("fgabcabcg")) // fabcg
