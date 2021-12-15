import java.io.File

data class Location(val x: Int, val y: Int)

enum class Direction(val x: Int, val y: Int) {
    UP(0, -1),
    DOWN(0, 1),
    LEFT(-1, 0),
    RIGHT(1, 0),
}

data class RiskMap(val data: List<Int>, val rows: Int, val cols: Int) {
    val start = Location(0,0)
    val end = Location(cols-1,rows-1)

    fun getRisk(location: Location) = data[location.y * cols + location.x]

    fun getNeighbours(location: Location): List<Location> =
        Direction.values()
            .filter {
                when (it) {
                    Direction.UP -> location.y > 0
                    Direction.DOWN -> location.y < this.rows - 1
                    Direction.LEFT -> location.x > 0
                    Direction.RIGHT -> location.x < this.cols - 1
                }
            }
            .map { Location(location.x + it.x, location.y + it.y) }
            .toList()

    fun getExtendedMap(x: Int, y: Int): RiskMap {
        val data = data.map { it - 1 }
        val extendedX = data.chunked(cols).flatMap { row -> (0 until x).flatMap { i -> row.map { (it + i) % 9 } } }
        val extendedY = (0 until y).flatMap { i -> extendedX.map { (it + i) % 9 } }
        val extended = extendedY.map { it + 1 }

        return RiskMap(extended, rows * y, cols * x)
    }

    companion object {
        fun fromFile(pathname: String): RiskMap {
            val rowData = File(pathname).useLines { lines ->
                lines.map { line ->
                    line.toCharArray().map { Character.getNumericValue(it) }
                }.toList()
            }

            return RiskMap(rowData.flatten(), rowData.count(), rowData[0].count())
        }
    }
}

fun findPath(map: RiskMap, start: Location, goal: Location): Int? = run {
    val cameFrom = mutableMapOf<Location, Location>()

    val risk = mutableMapOf(Pair(start, 0))
    val queue = mutableSetOf(start) // could use sortedSet for better performance

    while (queue.isNotEmpty()) {
        val current = queue.minByOrNull { risk[it]!! }!!
        if (current == goal) {
            return risk[current]
        }

        queue.remove(current)

        for (neighbour in map.getNeighbours(current)) {
            val knownRisk = risk[neighbour]
            val newRisk = risk[current]!! + map.getRisk(neighbour)

            if (knownRisk == null || newRisk < knownRisk) {
                risk[neighbour] = newRisk
                cameFrom[neighbour] = current
                queue.add(neighbour)
            }
        }
    }

    return null
}

fun main() {
    val map = RiskMap.fromFile("input-day-15.txt")

    val mapFull = map.getExtendedMap(5, 5)

    println("Task 1:")
    println(findPath(map, map.start, map.end))

    println("Task 2:")
    println(findPath(mapFull, mapFull.start, mapFull.end))
}
