import java.util.*

class AuthenticationManager(private val timeToLive: Int) {

    private val expireTime = ArrayDeque<Int>()
    private val renewTime = TreeMap<Int, Int>()
    private val tokens = mutableMapOf<String, Int>()

    private var currentTime = 0
    private var unexpiredTokens = 0

    fun generate(tokenId: String, currentTime: Int) {
        timeTo(currentTime)
        tokens[tokenId] = currentTime + timeToLive
        expireTime.addLast(currentTime + timeToLive)
        unexpiredTokens++
    }

    fun renew(tokenId: String, currentTime: Int) {
        timeTo(currentTime)
        if (tokenId !in tokens || tokens[tokenId]!! <= currentTime) {
            return
        }
        renewTime.putIfAbsent(tokens[tokenId]!!, 0)
        renewTime.compute(tokens[tokenId]!!) { _, v ->
            v!! + 1
        }
        tokens[tokenId] = currentTime + timeToLive
        expireTime.addLast(currentTime + timeToLive)
    }

    fun countUnexpiredTokens(currentTime: Int): Int {
        timeTo(currentTime)
        return unexpiredTokens
    }

    private fun timeTo(currentTime: Int) {
        this.currentTime = currentTime
        while (expireTime.isNotEmpty() && expireTime.first() <= currentTime) {
            expireTime.removeFirst()
            unexpiredTokens--
        }

        while (renewTime.isNotEmpty() && renewTime.firstKey() <= currentTime) {
            val entry = renewTime.firstEntry()
            renewTime.remove(entry.key)
            unexpiredTokens += entry.value
        }
    }

}