open System
open System.Collections.Generic
open System.IO

type Grid = Map<int * int, Map<int, int>>
type Direction = Left | Right | Up | Down

let parseInstruction (s: string) =
    let i = s.Substring(1) |> int
    match s.[0] with
    | 'L' -> (Left, i)
    | 'R' -> (Right, i)
    | 'U' -> (Up, i)
    | 'D' -> (Down, i)
    | _ -> failwithf "Invalid instruction %s" s

let incrementPosition (grid: Grid) (i, pos) : Grid =
    let counts = Map.tryFind pos grid |> Option.defaultValue Map.empty
    let newCounts =
        match Map.tryFind i counts with
        | Some n -> Map.add i (n + 1) counts
        | None -> Map.add i 1 counts
    Map.add pos newCounts grid

let applyInstructions (grid: Grid) (i, instructions) =
    let folder (grid: Grid, (x, y)) (dir, dist) =
        let (points, newPos) =
            match dir with
            | Left ->  (seq { for x in x - 1 .. -1 .. x - dist -> (i, (x, y)) }, (x - dist, y))
            | Right -> (seq { for x in x + 1 .. x + dist       -> (i, (x, y)) }, (x + dist, y))
            | Up ->    (seq { for y in y + 1 .. y + dist       -> (i, (x, y)) }, (x, y + dist))
            | Down ->  (seq { for y in y - 1 .. -1 .. y - dist -> (i, (x, y)) }, (x, y - dist))
        let newGrid = points |> Seq.fold incrementPosition grid
        (newGrid, newPos)
    instructions |> Seq.fold folder (grid, (0, 0)) |> fst

let manhattanDistance (x, y) =
    (abs x) + (abs y)

let findClosestPosition (grid: Grid) =
    grid
    |> Seq.filter (fun pair -> Map.count pair.Value > 1)
    |> Seq.map (fun pair -> pair.Key)
    |> Seq.minBy manhattanDistance

[<EntryPoint>]
let main argv =
    argv.[0]
    |> File.ReadAllLines
    |> Seq.mapi (fun i line -> (i, line.Split(',') |> Array.map parseInstruction))
    |> Seq.fold applyInstructions Map.empty
    |> findClosestPosition
    |> manhattanDistance
    |> printfn "%i" 
    0
