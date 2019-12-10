open System
open System.IO

type Grid = char array array

let indicies (grid: Grid) =
    seq {
        for y in 0 .. grid.Length - 1 do
            for x in 0 .. grid.[0].Length - 1 -> (x, y)
    }

let rec euclidsAlgorithm a b =
    match (a, b) with
    | (a, 0) -> a
    | (a, b) -> euclidsAlgorithm b (a % b)

let buildGrid lines =
    lines
    |> Seq.map Array.ofSeq
    |> Array.ofSeq

let countAsteroids (grid: Grid) (x: int, y: int) =
    grid
    |> indicies
    |> Seq.filter ((<>) (x, y))
    |> Seq.filter (fun (x1, y1) -> grid.[y1].[x1] = '#')
    |> Seq.map (fun (x1, y1) ->
        let (dx, dy) = (x1 - x, y1 - y)
        let gcd = abs (euclidsAlgorithm dx dy)
        (dx / gcd, dy / gcd))
    |> Set.ofSeq
    |> Set.count

let findSolution (grid: Grid) =
    let answers =
        grid
        |> indicies
        |> Seq.filter (fun (x, y) -> grid.[y].[x] = '#')
        |> Seq.map (fun pos -> (pos, countAsteroids grid pos))
    answers |> Seq.maxBy snd

[<EntryPoint>]
let main argv =
    argv.[0]
    |> File.ReadLines
    |> buildGrid
    |> findSolution
    |> printfn "%A"
    0
