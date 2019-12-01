open System.IO

let getFuelRequired n = (n / 3) - 2

[<EntryPoint>]
let main argv =
    argv.[0]
    |> File.ReadLines
    |> Seq.sumBy (int >> getFuelRequired)
    |> printfn "%i"
    0
