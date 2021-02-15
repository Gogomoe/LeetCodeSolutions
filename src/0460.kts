class LFUCache(val capacity: Int) {

    class Chain(val count: Int) {
        val head: Node = Node(-1, -1, this)
        val tail: Node = Node(-2, -2, this)

        init {
            head.next = tail
            tail.prev = head
        }

        var nextChain: Chain? = null

        override fun toString(): String {
            val list = mutableListOf<Node>()
            var node = head.next!!
            while (node != tail) {
                list.add(node)
                node = node.next!!
            }
            return "Chain($count)[${list.joinToString()}]"
        }

    }

    class Node(val key: Int, var value: Int, var chain: Chain) {
        var prev: Node? = null
        var next: Node? = null

        override fun toString(): String {
            return "Node($key)"
        }

    }

    val values = mutableMapOf<Int, Node>()
    var one = Chain(1)
    var least = one

    fun get(key: Int): Int {
        if (!values.containsKey(key)) {
            return -1
        }
        val node = values[key]!!
        increaseCount(node)
        return node.value
    }

    fun put(key: Int, value: Int) {
        if (capacity == 0) {
            return
        }
        if (values.containsKey(key)) {
            val node = values[key]!!
            increaseCount(node)
            node.value = value
            return
        }
        if (values.size == capacity) {
            removeLFUItem()
        }
        val node = Node(key, value, one)

        val firstNode = one.head.next!!
        firstNode.prev = node
        node.next = firstNode
        node.prev = one.head
        one.head.next = node

        values[key] = node
        least = one
    }

    private fun increaseCount(node: Node) {
        val chain = node.chain
        if (chain.nextChain == null) {
            chain.nextChain = Chain(chain.count + 1)
        }
        moveItemToChain(node, chain.nextChain!!)

        if (chain == least && chain.head.next == chain.tail) {
            least = chain.nextChain!!
        }
    }

    private fun moveItemToChain(node: Node, nextChain: Chain) {
        val prev = node.prev!!
        val next = node.next!!
        prev.next = next
        next.prev = prev

        val firstNode = nextChain.head.next!!
        firstNode.prev = node
        node.next = firstNode
        node.prev = nextChain.head
        nextChain.head.next = node

        node.chain = nextChain
    }

    private fun removeLFUItem() {
        val next = least.tail
        val toRemove = next.prev!!
        val prev = toRemove.prev!!

        prev.next = next
        next.prev = prev

        values.remove(toRemove.key)
    }

}

/**
 * Your LFUCache object will be instantiated and called as such:
 * var obj = LFUCache(capacity)
 * var param_1 = obj.get(key)
 * obj.put(key,value)
 */

val cache = LFUCache(2)
cache.put(1, 1)
cache.put(2, 2)
cache.get(1)
cache.put(3, 3)
cache.get(2)
cache.get(3)
cache.put(4, 4)
cache.get(1)
cache.get(3)
cache.get(4)