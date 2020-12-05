open System
open System.IO

type Tile = Open | Tree

let parseLine line =
    let parseTile c =
        match c with
        | '.' -> Open
        | '#' -> Tree
        | _ -> failwithf "Invalid tile found: %c" c
    line |> Array.ofSeq |> Array.map parseTile

let idx (grid: Tile array array) x y = 
    match Array.tryItem y grid with
    | Some row -> Some row.[x % row.Length]
    | None -> None

let countSlope grid dx dy =
    let rec inner acc x y =
        match idx grid x y with
        | Some Open -> inner acc (x + dx) (y + dy)
        | Some Tree -> inner (acc + 1) (x + dx) (y + dy)
        | None -> acc
    inner 0 0 0

[<EntryPoint>]
let main argv =
    let grid = 
        argv.[0]
        |> File.ReadAllLines
        |> Array.map parseLine
    [ (1, 1); (3, 1); (5, 1); (7, 1); (1, 2) ]
    |> Seq.map (fun (dx, dy) -> countSlope grid dx dy)
    |> Seq.fold (*) 1
    |> printfn "%A"
    0
