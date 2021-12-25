open System
open System.IO

module Seq =
    let rec any s =
        if Seq.isEmpty s then false
        elif Seq.head s then true
        else s |> Seq.tail |> any

    let rec all s =
        if Seq.isEmpty s then true
        elif not (Seq.head s) then false
        else s |> Seq.tail |> all
    
    let continuousMap f init s =
        let rec inner acc s =
            seq {
                match Seq.tryHead s with
                | Some head ->
                    let acc = f head acc
                    yield acc
                    yield! s |> Seq.tail |> inner acc
                | None -> ()
            }
        inner init s

type Grid = (int * bool) array array

type Solved =
    { WinningNumber: int
      Grid: Grid }
and State =
    | Solved of Solved
    | Unsolved of Grid

let isSolved state =
    match state with
    | Solved _ -> true
    | Unsolved _ -> false

let parseLine (sep: char) (line: string) =
    line.Split(sep, StringSplitOptions.RemoveEmptyEntries)
    |> Array.map int

let parseGrid (lines: string array) =
    let size = lines |> Array.tryHead |> Option.map (parseLine ' ' >> Array.length)
    if size.IsNone || Array.length lines < size.Value then
        None
    else
        lines
        |> Array.take size.Value
        |> Array.map (parseLine ' ')
        |> Array.map (Array.map (fun i -> i, false))
        |> Some

let mark i (grid: Grid) =
    grid
    |> Array.map (Array.map (fun (j, b) -> if i = j then (j, true) else (j, b)))

let row (grid: Grid) i =
    grid[i] |> Seq.map snd |> Seq.all

let col (grid: Grid) i =
    grid |> Seq.map (fun row -> snd row[i]) |> Seq.all

let winner (grid: Grid) =
    let indexes = seq { for i in 0 .. (grid.Length - 1) -> i }
    indexes |> Seq.map (row grid) |> Seq.any
    || indexes |> Seq.map (col grid) |> Seq.any

let findWinners (rng: int array) (grid: Grid) =
    let mapper i (states: State) : State =
        match states with
        | Solved s -> Solved s
        | Unsolved grid ->
            let grid = mark i grid
            if winner grid then Solved { WinningNumber = i; Grid = grid }
            else Unsolved grid
    rng |> Seq.continuousMap mapper (Unsolved grid)

let parseInput (lines: string array) : (int array * Grid array) =
    let rng = lines[0] |> parseLine ','
    let grids =
        lines
        |> Array.skip 1
        |> Array.chunkBySize 6
        |> Array.choose (fun chunk -> chunk |> Array.skip 1 |> parseGrid)
    (rng, grids)

let score state =
    state.WinningNumber * (state.Grid
    |> Seq.collect id
    |> Seq.filter (snd >> not)
    |> Seq.map fst
    |> Seq.sum)

let args = Environment.GetCommandLineArgs()
let lines = args[1] |> File.ReadAllLines

let (rng, grids) =
    lines
    |> parseInput

let result =
    grids
    |> Array.map (findWinners rng >> Array.ofSeq)
    |> Seq.maxBy (fun states -> states |> Seq.findIndex isSolved)
    |> Seq.last

match result with
| Unsolved _ -> failwith "impossible"
| Solved solved ->
    score solved
    |> printfn "%A"
