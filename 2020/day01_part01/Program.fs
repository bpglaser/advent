open System
open System.IO

let solve nums =
    let rec inner seen nums =
        match nums with
        | x :: xs ->
            let complement = 2020 - x
            if Set.contains complement seen then
                Some (x * complement)
            else
                inner (Set.add x seen) xs
        | [] ->
            None
    inner Set.empty nums

[<EntryPoint>]
let main argv =
    argv.[0]
    |> File.ReadAllLines
    |> Seq.map int
    |> List.ofSeq
    |> solve
    |> printfn "%A"
    0