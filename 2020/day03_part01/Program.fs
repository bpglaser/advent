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

let countSlope grid =
    let rec inner acc x y =
        match idx grid x y with
        | Some Open -> inner acc (x + 3) (y + 1)
        | Some Tree -> inner (acc + 1) (x + 3) (y + 1)
        | None -> acc
    inner 0 0 0

[<EntryPoint>]
let main argv =
    argv.[0]
    |> File.ReadAllLines
    |> Array.map parseLine
    |> countSlope
    |> printfn "%A"
    0