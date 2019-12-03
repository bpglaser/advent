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

let setPositionValue (grid: Grid) (lineNumber, pos, signal) =
    let counts = Map.tryFind pos grid |> Option.defaultValue Map.empty
    let newCounts =
        match Map.tryFind lineNumber counts with
        | Some n -> Map.add lineNumber (min n signal) counts
        | None -> Map.add lineNumber signal counts
    Map.add pos newCounts grid

let applyInstructions (grid: Grid) (lineNumber: int, instructions) =
    let folder (grid: Grid, (x, y), signal) (dir, dist) =
        let attachSignal = Seq.mapi (fun i (a, b) -> (a, b, signal + i + 1))

        let (points, newPos) =
            match dir with
            | Left ->
                (seq { for x in x - 1 .. -1 .. x - dist -> (lineNumber, (x, y)) } |> attachSignal,
                    (x - dist, y))
            | Right ->
                (seq { for x in x + 1 .. x + dist -> (lineNumber, (x, y)) } |> attachSignal,
                    (x + dist, y))
            | Up ->
                (seq { for y in y + 1 .. y + dist -> (lineNumber, (x, y)) } |> attachSignal,
                    (x, y + dist))
            | Down ->
                (seq { for y in y - 1 .. -1 .. y - dist -> (lineNumber, (x, y)) } |> attachSignal,
                    (x, y - dist))
        let newGrid = points |> Seq.fold setPositionValue grid
        (newGrid, newPos, signal + dist)
    instructions |> Seq.fold folder (grid, (0, 0), 0) |> (fun (a, _, _) -> a)

let findBestPosition (grid: Grid) =
    grid
    |> Seq.filter (fun pair -> Map.count pair.Value > 1)
    |> Seq.map (fun pair -> pair.Value |> Seq.sumBy (fun pair -> pair.Value))
    |> Seq.min

[<EntryPoint>]
let main argv =
    argv.[0]
    |> File.ReadAllLines
    |> Seq.mapi (fun i line -> (i, line.Split(',') |> Array.map parseInstruction))
    |> Seq.fold applyInstructions Map.empty
    |> findBestPosition
    |> printfn "%A" 
    0
