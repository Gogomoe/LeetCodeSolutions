class Solution {
    fun rankTeams(votes: Array<String>): String {
        val teams = Array(26) { 'A' + it to IntArray(26) { 0 } }

        for (vote in votes) {
            vote.forEachIndexed { index, char ->
                teams[char - 'A'].second[index]++
            }
        }

        return teams.sortedWith(Comparator { (_, a), (_, b) ->
            for (i in a.indices) {
                if (a[i] != b[i]) {
                    return@Comparator -(a[i] - b[i])
                }
            }
            0
        }).take(votes[0].length).joinToString("") { it.first.toString() }

    }
}

Solution().rankTeams(arrayOf("ABC", "ACB", "ABC", "ACB", "ACB"))
Solution().rankTeams(arrayOf("WXYZ","XYZW"))