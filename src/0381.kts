import java.util.*
import kotlin.collections.HashMap

class RandomizedCollection() {

    /** Initialize your data structure here. */

    private val list = mutableListOf<Int>()
    private val positions = HashMap<Int, LinkedList<Int>>()


    /** Inserts a value to the collection. Returns true if the collection did not already contain the specified element. */
    fun insert(`val`: Int): Boolean {
        val contains = positions.containsKey(`val`)
        positions.compute(`val`) { _, set ->
            (set ?: LinkedList<Int>()).also { it.add(list.size) }
        }
        list.add(`val`)
        return !contains
    }

    /** Removes a value from the collection. Returns true if the collection contained the specified element. */
    fun remove(`val`: Int): Boolean {
        if (!positions.containsKey(`val`)) {
            return false
        }
        val position = positions[`val`]!!.removeLast()!!
        if (positions[`val`]!!.isEmpty()) {
            positions.remove(`val`)
        }
        if (position != list.size - 1) {
            val move = list[list.size - 1]
            list[position] = move
            positions[move]!!.remove(list.size - 1)
            positions[move]!!.add(position)
        }
        list.removeAt(list.size - 1)
        return true
    }

    /** Get a random element from the collection. */
    fun getRandom(): Int {
        return list.random()
    }

}

/**
 * Your RandomizedCollection object will be instantiated and called as such:
 * var obj = RandomizedCollection()
 * var param_1 = obj.insert(`val`)
 * var param_2 = obj.remove(`val`)
 * var param_3 = obj.getRandom()
 */