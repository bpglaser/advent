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

let isHighPoint (x, y) (grid: Grid) = grid[y][x] = 9

let findLowPoints grid =
    let height = Array.length grid
    let width = Array.length grid[0]
    seq {
        for y in 0..(height - 1) do
            for x in 0..(width - 1) do
                if isLowPoint (x, y) grid then
                    yield x, y, grid[y][x]
    }

let findBasin lowPoint (grid: Grid) =
    let rec bfs queue seen =
        match queue with
        | [] -> seen
        | x::xs ->
            let unseen =
                neighbors x grid
                |> Seq.filter (fun pos -> not (isHighPoint pos grid))
                |> Seq.filter (fun pos -> not (Set.contains pos seen))
                |> List.ofSeq
            bfs (xs @ unseen) (Set.union seen (Set.ofList unseen))
    bfs [lowPoint] (Set.singleton lowPoint)

let riskLevel (x, y, height) = 1 + height

let solve grid =
    findLowPoints grid
    |> Seq.map (fun (x, y, _) -> findBasin (x, y) grid |> Set.count)
    |> Seq.sortDescending
    |> Seq.take 3
    |> Seq.fold ((*)) 1

[<EntryPoint>]
let main args =
    let charToInt c = int c - int '0'

    args[0]
    |> File.ReadAllLines
    |> Array.map (Seq.map charToInt >> Array.ofSeq)
    |> solve
    |> printfn "%A"

    0
