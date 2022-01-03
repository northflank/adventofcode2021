import scala.collection.{View, mutable}

case class Amphipod(kind: Int, id: Int) {
  def moveCost: Int =
    kind match {
      case 0 => 1
      case 1 => 10
      case 2 => 100
      case 3 => 1000
    }
}

object Amphipod {
  private val counter = Iterator.from(0)

  def apply(kind: String): Amphipod = {
    new Amphipod(kind(0) - 'A', counter.next())
  }
}

trait Location

case class Hallway(i: Int) extends Location

case class Room(i: Int, s: Int) extends Location

/** Captures a configuration of amphipods and their positions in the cave */
case class State private (
    hallway: IndexedSeq[Option[Amphipod]],
    rooms: IndexedSeq[IndexedSeq[Option[Amphipod]]]
) {
  import State._

  /** Lists all amphipods together with their current location */
  def amphipods: View[(Option[Amphipod], Location)] =
    hallway.zipWithIndex.view.map { case (a, i) => (a, Hallway(i)) } ++
      rooms.zipWithIndex.view.flatMap {
        case (r, i) =>
          r.zipWithIndex.view.map { case (a, j) => (a, Room(i, j)) }
      }

  /** Iterate neighbouring states together with associated transition cost */
  def nextStates(): IndexedSeq[(State, Int)] = {
    val result =
      IndexedSeq
        .newBuilder[(Amphipod, Location, Location)] // amphipod, from, to

    /** Iterate legal moves from a hallway position */
    def addMovesFromHallway(
        amphipod: Amphipod,
        start: Location,
        locations: Iterator[Int]
    ): Unit = {
      def storeLegalHallwayMoves(p1: Int): Unit = {
        (hallwayToRoom(p1), start) match {
          case (None, Room(_, _)) =>
            // move from room to a legal hallway position
            result += ((amphipod, start, Hallway(p1)))

          case (Some(roomIndex), _) if roomIndex == amphipod.kind =>
            // move into a legal room
            val room = rooms(roomIndex)
            for ((r, i) <- room.zipWithIndex) {
              r match {
                case None =>
                  result += ((amphipod, start, Room(roomIndex, i)))
                  return
                case Some(a1) if a1.kind != amphipod.kind =>
                  return
                case Some(a1) if a1.kind == amphipod.kind =>
              }
            }

          case _ => // no legal moves possible
        }
      }

      locations
        .takeWhile(hallway.indices.contains(_))
        .takeWhile(hallway(_).isEmpty)
        .foreach(p1 => {
          storeLegalHallwayMoves(p1)
        })
    }

    // iterate moves from hallway to rooms
    hallway.zipWithIndex.view.filter(_._1.isDefined).foreach {
      case (Some(a), pos) =>
        addMovesFromHallway(a, Hallway(pos), Iterator.from(pos - 1, -1))
        addMovesFromHallway(a, Hallway(pos), Iterator.from(pos + 1))
    }

    // iterate moves from rooms to hallway / rooms
    rooms.zipWithIndex.view.foreach {
      case (roomArray, roomId) =>
        var last: Option[(Amphipod, Room)] = None

        for ((a, roomPos) <- roomArray.zipWithIndex) {
          val room = Room(roomId, roomPos)
          (last, a) match {
            case (Some(_), Some(a))                  => last = Some((a, room))
            case (Some(_), None)                     =>
            case (None, Some(a)) if a.kind != roomId => last = Some((a, room))
            case (None, Some(a)) if a.kind == roomId =>
            case (None, None)                        =>
          }
        }

        last match {
          case Some((a, room)) =>
            val hallwayIdx = roomToHallway(room.i)
            addMovesFromHallway(a, room, Iterator.from(hallwayIdx - 1, -1))
            addMovesFromHallway(a, room, Iterator.from(hallwayIdx + 1))
          case _ =>
        }
    }

    result.result().map {
      case (a, from, to) =>
        (move(from, to), getDistance(from, to) * a.moveCost)
    }
  }

  /** Creates a new state given a move */
  def move(from: Location, to: Location): State = {
    var hallway = this.hallway
    var rooms = this.rooms

    val amphipod = from match {
      case Hallway(i) =>
        hallway = hallway.updated(i, None)
        this.hallway(i)
      case Room(a, b) =>
        rooms = rooms.updated(a, rooms(a).updated(b, None))
        this.rooms(a)(b)
    }

    to match {
      case Hallway(i) =>
        hallway = hallway.updated(i, amphipod)
      case Room(a, b) =>
        rooms = rooms.updated(a, rooms(a).updated(b, amphipod))
    }

    State(hallway, rooms)
  }

  /** Computes the distance between two locations */
  def getDistance(from: Location, to: Location): Int = {
    (from, to) match {
      case (Hallway(a), Hallway(b))           => Math.abs(b - a)
      case (Room(a, i), Room(b, j)) if a == b => Math.abs(i - j)
      case (Room(a, i), Room(b, j)) if a != b =>
        getDistance(Room(a, i), Hallway(roomToHallway(a))) +
          getDistance(Room(b, j), Hallway(roomToHallway(a)))

      case (Room(a, i), Hallway(b)) =>
        val exit = roomToHallway(a)
        (rooms(a).length - i) + getDistance(Hallway(exit), Hallway(b))

      case (hallway: Hallway, room: Room) => getDistance(room, hallway)
    }
  }

  /**
    * Estimates a lower bound for the energy required to transition to the solution state.
    * The estimate is obtained by summing the distances of all amphipods to their assigned
    * rooms multiplied by their respective energy requirements. By ignoring any collisions
    * and position assignments within target rooms, the approach underestimates the actual
    * remaining cost.
    */
  def estimateRemainingCost: Int = {
    amphipods
      .filter(_._1.isDefined)
      .map {
        case (Some(a), Room(i, _)) if a.kind == i => 0
        case (Some(a), loc) =>
          getDistance(
            loc,
            Room(a.kind, rooms(a.kind).length - 1)
          ) * a.moveCost
      }
      .sum
  }

  def isSolved: Boolean = {
    for ((room, i) <- rooms.zipWithIndex; a <- room) {
      if (a.isDefined && a.get.kind != i) {
        return false
      }
    }
    if (hallway.exists(_.isDefined)) {
      return false
    }
    true
  }

  def render(): String = {
    val str = new StringBuilder()

    str ++= "┌───────────┐" + "\n"
    str ++= "│"

    def toSymbol(amphipod: Option[Amphipod]): String = {
      amphipod match {
        case Some(a) => ('A' + a.kind).toChar.toString
        case None    => "."
      }
    }

    str ++= hallway.map(toSymbol).mkString("")
    str ++= "|\n"

    rooms.transpose.view.reverse.zipWithIndex.foreach {
      case (r, i) =>
        def select(i: Int, a: String, b: String) =
          if (i == 0) { a }
          else { b }

        val start = select(i, "└─┐", "  │")
        val end = select(i, "┌─┘", "│")

        str ++= r.map(toSymbol).mkString(start, "│", end)
        str ++= "\n"
    }
    str ++= "  └─┴─┴─┴─┘"

    str.result()
  }
}

object State {
  private lazy val roomEntrances = Array(2, 4, 6, 8)

  def roomToHallway(room: Int): Int = (room + 1) * 2
  def hallwayToRoom(hallway: Int): Option[Int] = {
    if (roomEntrances.contains(hallway)) {
      Some(hallway / 2 - 1)
    } else {
      None
    }
  }

  def apply(rooms: String*): State = {
    new State(
      IndexedSeq.fill(11)(None),
      rooms.toIndexedSeq.map(r =>
        r.toIndexedSeq.map(a => Some(Amphipod(a.toString)))
      )
    )
  }
}

object Day23 extends App {

  /**
    * A-Star search on [[State.nextStates]] using [[State.estimateRemainingCost]] as
    * underestimating heuristic for remaining costs.
    *
    * @see [[https://en.wikipedia.org/wiki/A*_search_algorithm]]
    */
  def solve(initialState: State): Option[Int] = {
    val gScore = mutable.Map((initialState, 0))
    val fScore = mutable.Map((initialState, initialState.estimateRemainingCost))

    val nextStates = mutable.PriorityQueue(
      (initialState, 0, initialState.estimateRemainingCost)
    )(Ordering.by(-_._3))

    val cameFrom = mutable.Map.empty[(State, Int), (State, Int)]

    val counter = Iterator.from(1)
    while (nextStates.nonEmpty) {
      val (state, g1, f1) = nextStates.dequeue()

      // Since we allow duplicates in the priority queue we only process entries
      // with up-to-date g- and f-scores
      if (f1 == fScore(state) && g1 == gScore(state)) {
        val cost = gScore(state)

        if (state.isSolved) {
          return Some(cost)
        }

        for ((newState, moveCost) <- state.nextStates()) {
          val newCost = cost + moveCost

          cameFrom.put((newState, moveCost), (state, cost))

          if (!gScore.contains(newState) || newCost < gScore(newState)) {
            val newEstimatedTotal = newCost + newState.estimateRemainingCost

            gScore(newState) = newCost
            fScore(newState) = newEstimatedTotal

            nextStates.enqueue((newState, newCost, newEstimatedTotal))
          }
        }
      }
    }

    // No solution found
    None
  }

  // Task 1
  val initialState1 = State(
    "BD",
    "CD",
    "AB",
    "CA"
  )

  println("Task 1:")
  println(solve(initialState1).getOrElse("no solution"))
  println()

  // Task 2
  val initialState2 = State(
    "BDDD",
    "CBCD",
    "AABB",
    "CCAA"
  )

  println("Task 2:")
  println(solve(initialState2).getOrElse("no solution"))

}
