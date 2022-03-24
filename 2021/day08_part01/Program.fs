open System.IO

type Input =
    { Patterns: string array
      Output: string array }

let guessDigit s =
    match String.length s with
    | 2 -> Some 1
    | 4 -> Some 4
    | 3 -> Some 7
    | 7 -> Some 8
    | _ -> None

let parse (line: string) =
    match line.Split('|') with
    | [| patterns; output |] ->
        { Patterns = patterns.Split(' ')
          Output = output.Split(' ') }
    | _ -> failwithf "invalid line: %s" line

let solve inputs =
    inputs
    |> Seq.sumBy (fun input ->
        input.Output
        |> Seq.filter (guessDigit >> Option.isSome)
        |> Seq.length)

[<EntryPoint>]
let main args =
    args[0]
    |> File.ReadAllLines
    |> Seq.map parse
    |> solve
    |> printfn "%A"

    0
