class Solution {
    fun countPrimeSetBits(L: Int, R: Int): Int {
        val prime = booleanArrayOf(
            false, false, true, true, false, //2, 3
            true, false, true, false, false, //5, 7
            false, true, false, true, false, //11, 13
            false, false, true, false, true  //17, 19
        )
        return (L..R).map(Integer::bitCount).map { prime[it] }.count { it }
    }

}