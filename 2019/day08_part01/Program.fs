open System
open System.IO

let solve (width, height) (s: int seq) =
    let bestLayer = 
        s
        |> Seq.chunkBySize (width * height)
        |> Seq.map (Seq.countBy id)
        |> Seq.minBy (Seq.tryFind (fst >> ((=) 0)) >> Option.map snd >> Option.defaultValue 0)
        |> Map.ofSeq
    (Map.find 1 bestLayer) * (Map.find 2 bestLayer)

[<EntryPoint>]
let main argv =
    argv.[0]
    |> File.ReadLines
    |> Seq.head
    |> Seq.map (string >> Int32.Parse)
    |> solve (25, 6)
    |> printfn "%i"
    0
