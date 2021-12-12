import java.nio.file.Files
import java.nio.file.Path
import scala.collection.mutable;

case class Cave(id: String)(caveSystem: CaveSystem) {
  def isBig = id.toUpperCase == id
  def isSmall = !isBig
  def isStart = id == "start"
  def isEnd = id == "end"
  def neighbours = caveSystem.neighbours(this)
}

class CaveSystem(caveGraph: Map[String, Set[String]]) {
  def start: Cave = newCave("start")
  def caves: Iterable[Cave] = caveGraph.keys.map(newCave)
  def neighbours(cave: Cave): Set[Cave] = neighbours(cave.id)
  def neighbours(cave: String): Set[Cave] = caveGraph(cave).map(newCave)
  private def newCave(id: String): Cave = Cave(id)(this)
}

object CaveSystem {
  def fromFile(fileName: String): CaveSystem = {
    val mapBuilder = mutable.Map.empty[String, mutable.Set[String]]

    val lines = Files.readString(Path.of(fileName)).split("\n")

    lines.foreach(a => {
      a.split("-") match {
        case Array(from, to) =>
          mapBuilder.getOrElseUpdate(to, mutable.Set.empty) += from
          mapBuilder.getOrElseUpdate(from, mutable.Set.empty) += to
      }
    })

    new CaveSystem(mapBuilder.view.mapValues(_.toSet).toMap)
  }
}

object Day12 extends App {
  val caveSystem = CaveSystem.fromFile("input-day12.txt")

  def countPaths(
      cave: Cave,
      allowDoubleVisit: Boolean,
      visitedCaves: Map[Cave, Int] = Map.empty.withDefaultValue(0)
  ): Int = {
    if (cave.isEnd) {
      return 1
    }

    var count = 0;
    for (n <- cave.neighbours) if (!n.isStart) {
      val pastVisitCount = visitedCaves(n)
      val didDoubleVisit = visitedCaves.exists(n => n._1.isSmall && n._2 == 2)
      val allowedPastVisitCount =
        if (allowDoubleVisit && !didDoubleVisit) 1 else 0

      if (n.isBig || pastVisitCount <= allowedPastVisitCount) {
        count += countPaths(
          n,
          allowDoubleVisit,
          visitedCaves + ((n, pastVisitCount + 1))
        )
      }
    }

    count
  }

  println("Task 1", countPaths(caveSystem.start, allowDoubleVisit = false))
  println("Task 2", countPaths(caveSystem.start, allowDoubleVisit = true))
}
