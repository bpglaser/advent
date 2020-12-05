open System
open System.IO

let solve (nums: int array) =
    let options = seq {
        for i in 0..(nums.Length - 1) do
            for j in (i + 1)..(nums.Length - 1) do
                for k in (j + 1)..(nums.Length - 1) do
                    if nums.[i] + nums.[j] + nums.[k] = 2020 then
                        yield (nums.[i] * nums.[j] * nums.[k])
    }
    options |> Seq.tryHead

[<EntryPoint>]
let main argv =
    argv.[0]
    |> File.ReadAllLines
    |> Array.map int
    |> solve
    |> printfn "%A"
    0