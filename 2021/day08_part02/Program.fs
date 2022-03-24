open System
open System.IO

type Input =
    { Patterns: char Set array
      Output: char Set array }

let guessDigit s =
    match Set.count s with
    | 2 -> [ 1 ]
    | 3 -> [ 7 ]
    | 4 -> [ 4 ]
    | 5 -> [ 2; 3; 5 ]
    | 6 -> [ 0; 6; 9 ]
    | 7 -> [ 8 ]
    | _ -> failwithf "invalid pattern length: %A" s

let parse (line: string) =
    let refine (s: string) =
        s.Split(' ', StringSplitOptions.RemoveEmptyEntries)
        |> Array.map Set.ofSeq

    match line.Split('|') with
    | [| patterns; output |] ->
        { Patterns = refine patterns
          Output = refine output }
    | _ -> failwithf "invalid line: %s" line

let flip (a, b) = (b, a)

let refineMulti singles (pattern, digits) =
    let singles = singles |> Map.toSeq |> Seq.map flip |> Map.ofSeq
    let get n = Map.find n singles
    let containsDigit n = Set.isSubset (get n) pattern

    let intersectionCount n =
        Set.intersect (get n) pattern |> Set.count

    match digits with
    | [ 2; 3; 5 ] ->
        if containsDigit 7 then 3
        elif (intersectionCount 4) = 3 then 5
        else 2
    | [ 0; 6; 9 ] ->
        if containsDigit 4 then 9
        elif containsDigit 7 then 0
        else 6
    | _ -> failwithf "cannot refine pattern %A for possibilities %A" pattern digits

let solveSingle input =
    let (single, multi) =
        input.Patterns
        |> Array.map (fun pattern -> pattern, guessDigit pattern)
        |> Array.partition (fun (_, digits) -> List.length digits = 1)

    let single =
        single
        |> Seq.map (fun (pattern, digits) -> pattern, digits |> List.head)
        |> Map.ofSeq

    let multi =
        multi
        |> Seq.map (fun (pattern, digits) -> pattern, refineMulti single (pattern, digits))
        |> Map.ofSeq

    Seq.append (Map.toSeq single) (Map.toSeq multi)
    |> Map.ofSeq

let convert cypher input =
    input.Output
    |> Seq.map (fun pattern -> Map.find pattern cypher)
    |> Seq.fold (fun acc i -> acc * 10 + i) 0

let solve inputs =
    inputs
    |> Seq.sumBy (fun input -> convert (solveSingle input) input)

[<EntryPoint>]
let main args =
    args[0]
    |> File.ReadAllLines
    |> Seq.map parse
    |> solve
    |> printfn "%A"

    0
