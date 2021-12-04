open System.IO

let convert (s: string): int array =
    [| for c in s ->
        match c with
        | '0' -> 0
        | '1' -> 1
        | _ -> failwithf "invalid char %c" c |]

let countOnes (acc: int array) row =
    for (i, n) in Seq.indexed row do
        acc.[i] <- acc.[i] + n
    acc

let invert = Array.map (fun i -> ~~~i &&& 1)

let toInt = Array.fold (fun acc n -> (acc <<< 1) + n) 0

[<EntryPoint>]
let main argv =
    let lines = argv.[0] |> File.ReadAllLines
    let n = lines |> Array.length

    let counts =
        lines
        |> Seq.map convert
        |> Seq.fold countOnes [| for _ in lines.[0] -> 0 |]

    let majority =
        counts
        |> Array.map (fun i ->
            if i >= n / 2 then 1 else 0)

    let gamma = majority |> toInt

    let epsilon =
        majority
        |> invert
        |> toInt

    printfn "%A" (gamma * epsilon)
    0
