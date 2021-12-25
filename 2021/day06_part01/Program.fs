open System
open System.IO

let parse (line: string) =
    line.Split "," |> Array.map int |> List.ofArray

let advance fishes =
    let next i =
        if i = 0 then 6
        else i - 1
    let newCount = fishes |> Seq.filter ((=) 0) |> Seq.length
    let newFishes = List.replicate newCount 8
    (fishes |> List.map next) @ newFishes

let rec repeatedAdvance count fishes =
    if count = 0 then fishes
    else repeatedAdvance (count - 1) (advance fishes)

Environment.GetCommandLineArgs()[1]
|> File.ReadAllLines
|> Array.head
|> parse
|> repeatedAdvance 80
|> List.length
|> printfn "%A"
