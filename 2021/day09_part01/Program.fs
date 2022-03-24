open System.IO

type Grid = int array array

let neighbors (x, y) (grid: Grid) =
    let withinBounds (x, y) =
        if x < 0 || x >= Array.length grid[0] then false
        elif y < 0 || y >= Array.length grid then false
        else true
    seq { 0, -1; -1, 0; 1, 0; 0, 1 }
    |> Seq.map (fun (dx, dy) -> x + dx, y + dy)
    |> Seq.filter withinBounds

let isLowPoint (x, y) (grid: Grid) =
    let h = grid[y][x]
    neighbors (x, y) grid |> Seq.forall (fun (i, j) -> h < grid[j][i])

let findLowPoints grid =
    let height = Array.length grid
    let width = Array.length grid[0]
    seq {
        for y in 0..(height - 1) do
            for x in 0..(width - 1) do
                if isLowPoint (x, y) grid then
                    yield x, y, grid[y][x]
    }

let riskLevel (x, y, height) = 1 + height

let solve = findLowPoints >> Seq.sumBy riskLevel

[<EntryPoint>]
let main args =
    let charToInt c = int c - int '0'

    args[0]
    |> File.ReadAllLines
    |> Array.map (Seq.map charToInt >> Array.ofSeq)
    |> solve
    |> printfn "%A"

    0
