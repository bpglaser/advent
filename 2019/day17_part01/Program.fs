open System.IO
open Intcode

type Grid = Map<int * int, char>

let createGrid result =
    let rec inner buf result =
        match result with
        | AwaitingInput cont -> failwithf "Unexpected read"
        | Complete -> [ buf ]
        | HasOutput output ->
            let c =
                output.Output
                |> int
                |> char
            match c with
            | '\n' -> (List.rev buf) :: inner [] (output.Continue())
            | _ -> inner (c :: buf) (output.Continue())

    result
    |> inner []
    |> Seq.mapi (fun y row -> row |> Seq.mapi (fun x c -> ((x, y), c)))
    |> Seq.collect id
    |> Map.ofSeq

let isIntersection (grid: Grid) (x, y) =
    let matches =
        seq {
            (x - 1, y)
            (x + 1, y)
            (x, y - 1)
            (x, y + 1)
        }
        |> Seq.choose (fun pos -> Map.tryFind pos grid)
        |> Seq.filter (fun c -> c = '#')
        |> Seq.length
    matches = 4

let findScaffoldIntersections (grid: Grid) =
    grid
    |> Seq.choose (fun entry ->
        if entry.Value = '#' then Some entry.Key
        else None)
    |> Seq.filter (isIntersection grid)

let printGrid (grid: Grid) =
    let maxX =
        grid
        |> Seq.map (fun entry -> fst entry.Key)
        |> Seq.max

    let maxY =
        grid
        |> Seq.map (fun entry -> snd entry.Key)
        |> Seq.max

    for y in 0 .. maxY do
        for x in 0 .. maxX do
            printf "%c" (Map.find (x, y) grid)
        printfn ""

let findSolution line =
    let grid =
        line
        |> createState
        |> runIntCode
        |> createGrid

    printGrid grid

    grid
    |> findScaffoldIntersections
    |> Seq.sumBy (fun (a, b) -> a * b)

[<EntryPoint>]
let main argv =
    argv.[0]
    |> File.ReadLines
    |> Seq.head
    |> findSolution
    |> printfn "%A"
    0
