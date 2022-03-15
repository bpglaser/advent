open System
open System.IO

let parse (line: string) =
    line.Split ","
    |> Seq.map int
    |> Seq.countBy id
    |> Seq.map (fun (a, b) -> a, bigint b)
    |> Map.ofSeq

let inc v entry =
    match entry with
    | Some i -> Some(bigint.Add(i, v))
    | None -> Some v

let rec solve count fishes =
    if count = 0 then
        fishes |> Map.values |> Seq.sum
    else
        let next fishes k v =
            if k = 0 then
                fishes
                |> Map.change 6 (inc v)
                |> Map.change 8 (inc v)
            else
                fishes |> Map.change (k - 1) (inc v)

        let fishes = fishes |> Map.fold next Map.empty
        solve (count - 1) fishes

Environment.GetCommandLineArgs()[1]
|> File.ReadAllLines
|> Array.head
|> parse
|> solve 256
|> printfn "%A"
