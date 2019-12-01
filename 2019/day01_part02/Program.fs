open System.IO

let rec getFuelRequired n =
    let fuel = (n / 3) - 2
    if fuel <= 0 then
        0
    else
        fuel + getFuelRequired fuel

[<EntryPoint>]
let main argv =
    argv.[0]
    |> File.ReadLines
    |> Seq.sumBy (int >> getFuelRequired)
    |> printfn "%i"
    0
