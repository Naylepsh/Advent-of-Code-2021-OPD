import scala.util.chaining.*

def solve(filename: String, windowSize: Int) =
  io.Source.fromFile(filename)
    .getLines
    .map(_.toInt)
    .sliding(windowSize)
    .map(_.sum)
    .sliding(2)
    .filter:
      case Seq(a, b) => a < b
    .length
    .pipe(println)

val filename = "../day1/input.txt"
solve(filename, 2)
solve(filename, 3)
