open System
open System.IO
open System.Text.RegularExpressions
open System.Text

type Point =
    { X: int
      Y: int }

type Line =
    { P1: Point
      P2: Point }

type Grid = Map<Point, int>

let pattern = Regex "(\d+),(\d+) -> (\d+),(\d+)"

let parse line =
    let m = pattern.Match line
    let nums =
        m.Groups
        |> Seq.skip 1
        |> Seq.map (fun group -> group.Value |> int)
        |> Array.ofSeq
    { P1 = { X = nums[0]; Y = nums[1] }
      P2 = { X = nums[2]; Y = nums[3] } }

let isHorizontal line = line.P1.Y = line.P2.Y

let isVertical line = line.P1.X = line.P2.X

let range a b =
    seq { if a < b then for i in a .. b -> i else for i in a .. -1 .. b -> i }

let intermediaryPoints line : Point seq =
    if isHorizontal line || isVertical line then
        seq {
            for y in range line.P1.Y line.P2.Y do
                for x in range line.P1.X line.P2.X do
                    yield { X = x; Y = y }
        }
    else
        seq {
            for (x, y) in Seq.zip (range line.P1.X line.P2.X) (range line.P1.Y line.P2.Y) ->
                { X = x; Y = y }
        }

let fill grid line =
    line
    |> intermediaryPoints
    |> Seq.fold (fun grid p ->
        Map.change p (fun count ->
            match count with
            | Some count -> Some(count + 1)
            | None -> Some(1)) grid) grid

let lineToString line =
    sprintf "%d,%d -> %d,%d" line.P1.X line.P1.Y line.P2.X line.P2.Y

let gridToString (grid: Grid) =
    let keys = grid |> Map.keys
    let (minX, minY) = (keys |> Seq.map (fun p -> p.X) |> Seq.min, keys |> Seq.map (fun p -> p.Y) |> Seq.min)
    let (maxX, maxY) = (keys |> Seq.map (fun p -> p.X) |> Seq.max, keys |> Seq.map (fun p -> p.Y) |> Seq.max)
    let sb = StringBuilder()
    for y in minY .. maxY do
        for x in minX .. maxX do
            sb.Append(grid |> Map.tryFind { X = x; Y = y } |> Option.map string |> Option.defaultValue ".") |> ignore
        sb.AppendLine() |> ignore
    sb.ToString()


Environment.GetCommandLineArgs()[1]
|> File.ReadAllLines
|> Array.map parse
|> Array.fold fill Map.empty
|> Seq.filter (fun kv -> kv.Value > 1)
|> Seq.length
|> printfn "%i"
