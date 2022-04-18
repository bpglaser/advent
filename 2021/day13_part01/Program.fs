open System
open System.IO

type Fold =
    | XFold of int
    | YFold of int

let xPrefix = "fold along x="
let yPrefix = "fold along y="

let parse lines =
    let pair (s: string) =
        match s.Split(',', StringSplitOptions.TrimEntries) with
        | [| x; y |] -> Some(int x, int y)
        | _ -> None

    let fold (s: string) =
        if s.StartsWith xPrefix then
            s.Substring xPrefix.Length |> int |> XFold |> Some
        elif s.StartsWith yPrefix then
            s.Substring yPrefix.Length |> int |> YFold |> Some
        else
            None

    Array.choose pair lines, Array.choose fold lines

let applyFold fold points =
    match fold with
    | XFold i ->
        points
    | YFold i ->
        points

let solve (points, folds) =
    applyFold (Array.head folds) points
    |> Array.length

[<EntryPoint>]
let main args =
    args[0]
    |> File.ReadAllLines
    |> parse
    |> solve
    |> printfn "%A"

    0
