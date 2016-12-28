package day11_part01

import java.io.File

fun main(args: Array<String>) {
    val input = File(args.firstOrNull() ?: "input/day11_part01.txt").readLines()
    val checkedStates = hashSetOf<ShortRepresentation>()
    val stack = arrayListOf(parseInitialState(input))

    try {
        while (stack.isNotEmpty()) {
            if (checkedStates.size % 1000 == 0) {
                println("Stack size: ${stack.size}\tChecked states: ${checkedStates.size}")
            }
            val workingState = stack.removeAt(0)
            val possibleChildStates = workingState.getPossibleChildStates()
            for (state in possibleChildStates) {
                if (state.toShortRepresentation() !in checkedStates) {
                    if (state.isSolved) {
                        throw SolvedException(state)
                    }
                    stack += state
                    checkedStates += state.toShortRepresentation()
                }
            }
        }
        println("Failed to solve!")
    } catch (e: SolvedException) {
        println("Solved!")
        val states = arrayListOf(e.state)
        while (true) {
            val state = states.last().parent ?: break
            states += state
        }
        println("Depth: ${states.size - 1}")
    }
}

fun parseInitialState(input: List<String>): State {
    val children = input.withIndex().flatMap { parseChildren(it.index, it.value) }.toSet()
    return State(0, children)
}

fun parseChildren(floor: Int, line: String): List<Child> {
    if (line.endsWith("contains nothing relevant.")) {
        return emptyList()
    }
    return line.substringAfterLast("contains a ").split("((, )?and)|,".toRegex()).map { parseChild(floor, it) }
}

fun parseChild(floor: Int, s: String): Child {
    val words = s.trim().split("\\h+".toRegex())
    val lastWord = words.last()
    val penultimateWord = words.reversed()[1]
    if (lastWord.contains("generator")) {
        return Generator(floor, penultimateWord)
    } else if (lastWord.contains("microchip")) {
        return Microchip(floor, penultimateWord.substringBefore('-'))
    } else {
        throw RuntimeException("Failed to parse: (floor=$floor) , (string=$s)")
    }
}

data class ShortRepresentation(val elevatorPosition: Int, val childState: List<Pair<Int, Int>>)

abstract class Child(var floor: Int, val type: String) {
    abstract fun clone(): Child
    abstract fun isFried(siblings: Set<Child>): Boolean
}

class Generator(floor: Int, type: String) : Child(floor, type) {
    override fun isFried(siblings: Set<Child>): Boolean = false

    override fun clone(): Generator {
        return Generator(floor, type)
    }

    override fun toString(): String {
        return "Gen{$type @ $floor}"
    }

    override fun equals(other: Any?): Boolean {
        if (this === other) {
            return true
        }
        val otherGenerator = other as? Generator ?: return false
        return this.floor == otherGenerator.floor && this.type == otherGenerator.type
    }

    override fun hashCode(): Int = listOf(javaClass.name, floor, type).hashCode()
}

class Microchip(floor: Int, type: String) : Child(floor, type) {
    override fun isFried(siblings: Set<Child>): Boolean {
        var adjacentToOtherGenerator = false
        siblings.filter { it.floor == this.floor }
                .filter { it is Generator }
                .forEach {
                    if (it.type == this.type) {
                        return false
                    }
                    adjacentToOtherGenerator = true
                }
        return adjacentToOtherGenerator
    }

    override fun clone(): Child {
        return Microchip(floor, type)
    }

    override fun toString(): String {
        return "Chip{$type @ $floor}"
    }

    override fun equals(other: Any?): Boolean {
        if (this === other) {
            return true
        }
        val otherMicrochip = other as? Microchip ?: return false
        return this.floor == otherMicrochip.floor && this.type == otherMicrochip.type
    }

    override fun hashCode(): Int = listOf(javaClass.name, floor, type).hashCode()
}


class State(val elevatorPosition: Int, val children: Set<Child>, var parent: State? = null) {
    val isSolved: Boolean
        get() = children.all { it.floor == 3 }

    private val isValid: Boolean
        get() {
            for (child in children) {
                if (elevatorPosition < 0 || elevatorPosition > 3) {
                    return false
                }
                if (child.isFried(children)) {
                    return false
                }
            }
            return true
//            return children.all { it.floor >= 0 } and children.all { it.floor <= 3 } and !children.any { it.isFried(children) }
        }

    fun getPossibleChildStates(): List<State> {
        val possibleUpStates = getPossibleChildStates(1)
        var possibleDownStates = emptyList<State>()
        if (occupiedFloorsBelow()) {
            possibleDownStates = getPossibleChildStates(-1)
        }

        val validChildStates = (possibleUpStates + possibleDownStates).filter { it.isValid }
        return validChildStates
    }

    fun getIndexPairs(): List<Pair<Int, Int>> {
        val pairs = arrayListOf<Pair<Int, Int>>()
        val stack = children.toList().clone().toMutableList()
        while (stack.isNotEmpty()) {
            val firstChild = stack.removeAt(0)
            val secondChild = stack.find { it.type == firstChild.type } ?: throw RuntimeException("Unable to find match for: $firstChild")
            stack.remove(secondChild)
            if (firstChild is Microchip) {
                pairs += firstChild.floor to secondChild.floor
            } else {
                pairs += secondChild.floor to firstChild.floor
            }
        }
        return pairs.sortedBy { it.second }.sortedBy { it.first }
    }

    fun toShortRepresentation(): ShortRepresentation {
        return ShortRepresentation(elevatorPosition, getIndexPairs())
    }

    private fun getPossibleChildStates(offset: Int): List<State> {
        val workingChildren = children.toList()
        val workingChildIndexes = workingChildren.withIndex().filter { it.value.floor == elevatorPosition }.map { it.index }

        val newElevatorPosition = elevatorPosition + offset
        val possibleChildStates = hashSetOf<State>()

        for (firstChildIndex in workingChildIndexes) {
            val loneChildSwap = workingChildren.clone()
            loneChildSwap[firstChildIndex].floor = newElevatorPosition
            possibleChildStates += State(newElevatorPosition, loneChildSwap.toSet(), this)

            for (secondChildIndex in workingChildIndexes) {
                if (firstChildIndex == secondChildIndex) {
                    continue
                }
                val dualChildSwap = workingChildren.clone()
                dualChildSwap[firstChildIndex].floor = newElevatorPosition
                dualChildSwap[secondChildIndex].floor = newElevatorPosition
                possibleChildStates += State(newElevatorPosition, dualChildSwap.toSet(), this)
            }
        }

        return possibleChildStates.toList()
    }

    private fun occupiedFloorsBelow(): Boolean = children.any { it.floor < this.elevatorPosition }

    override fun toString(): String {
        return "Elevator: $elevatorPosition Children: $children\n" + parent.toString()
    }

    override fun equals(other: Any?): Boolean {
        if (this === other) return true
        if (other !is State) return false

        if (elevatorPosition != other.elevatorPosition) return false
        if (children != other.children) return false

        return true
    }

    override fun hashCode(): Int {
        var result = elevatorPosition
        result = 31 * result + children.hashCode()
        return result
    }
}

private fun List<Child>.clone(): List<Child> = this.map(Child::clone)

class SolvedException(val state: State) : Exception()
