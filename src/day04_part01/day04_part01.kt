package day04_part01

import java.io.File

fun main(args: Array<String>) {
    val total = File(args.firstOrNull() ?: "input/day04_part01.txt")
            .readLines()
            .filter(::isValid)
            .map(::extractSectorID)
            .sum()

    println("Sector ID sum: $total")
}

fun isValid(s: String): Boolean {
    val calculatedChecksum = s.split('-')
            .dropLast(1)
            .flatMap { it.toCharArray().asIterable() }
            .groupBy { it }
            .asSequence()
            .sortedBy { it.key }
            .sortedByDescending { it.value.size }
            .take(5)
            .map { it.key }
            .joinToString("")

    val givenChecksum = s.substringAfter("[").substringBeforeLast("]")
    return calculatedChecksum == givenChecksum
}

fun extractSectorID(s: String): Int {
    val toInt = s.substringAfterLast("-").substringBeforeLast("[").toInt()
    return toInt
}