package day04_part02

import java.io.File

fun main(args: Array<String>) {
    val sector = File(args.firstOrNull() ?: "input/day04_part02.txt")
            .readLines()
            .filter(::isValid)
            .map { decrypt(it) to extractSectorID(it) }
            .first { it.first == "northpole object storage" }
            .second
    println("North Pole objects sector ID: $sector")
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

fun extractSectorID(s: String) = s.substringAfterLast("-").substringBeforeLast("[").toInt()

fun decrypt(s: String): String {
    val sectorID = extractSectorID(s)
    val words = s.split('-').dropLast(1)
    return words.map { shiftCypher(it, sectorID) }.joinToString(" ")
}

fun shiftCypher(word: String, offset: Int): String {
    val stringBuilder = StringBuilder()
    val letters = "abcdefghijklmnopqrstuvwxyz".toCharArray()
    for (c in word) {
        val i = letters.indexOf(c)
        val replacement = letters[(i + offset) % letters.size]
        stringBuilder.append(replacement)
    }
    return stringBuilder.toString()
}