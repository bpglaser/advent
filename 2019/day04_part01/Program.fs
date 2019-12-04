open System

module Seq =
    let rec any pred s =
        match Seq.tryHead s with
        | Some e when pred e -> true
        | Some _ -> any pred (Seq.tail s)
        | None -> false

    let none pred s =
        not (any pred s)

let parseInput (s: string) =
    match s.Split('-') |> Array.map int with
    | [| a; b |] -> (a, b)
    | _ -> failwithf "Invalid input %s" s

let getDigits = string >> Array.ofSeq >> Array.map int

let inline isSixDigits i =
    100000 <= i && i <= 999999

let hasAdjacentRepeat digits =
    seq { 1 .. Array.length digits - 1 }
    |> Seq.any (fun i -> digits.[i - 1] = digits.[i])

let isNonDecreasing digits =
    seq { 1 .. Array.length digits - 1 }
    |> Seq.none (fun i -> digits.[i - 1] > digits.[i])

let matchesAllRules i =
    if isSixDigits i then
        let digits = getDigits i
        hasAdjacentRepeat digits && isNonDecreasing digits
    else false

[<EntryPoint>]
let main argv =
    let (lo, hi) = parseInput argv.[0]
    seq { lo .. hi }
    |> Seq.filter matchesAllRules
    |> Seq.length
    |> printfn "%i"
    0
