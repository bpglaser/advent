open System
open System.IO

module Seq =
    let rec repeat n s =
        seq {
            if n > 0 then
                yield! s
                yield! repeat (n - 1) s
        }

let rec pattern i =
    seq {
        for _ in 1 .. i -> 0
        for _ in 1 .. i -> 1
        for _ in 1 .. i -> 0
        for _ in 1 .. i -> -1
        yield! pattern i
    }

let parseLine (s: string) = s |> Seq.map (fun c -> int c - int '0') |> Array.ofSeq

let applyPattern inputNumbers pattern =
    let total =
        Seq.zip inputNumbers pattern
        |> Seq.sumBy (fun (a, b) -> a * b)
    abs (total % 10)

let phase inputNumbers n =
    seq { 1 .. n }
    |> Seq.map (pattern >> Seq.skip 1 >> Seq.take n >> applyPattern inputNumbers)
    |> Array.ofSeq

let findSolution numPhases (line: string) =
    let startingInput = parseLine line
    let n = Seq.length startingInput

    let rec inner count inputNumbers =
        if count = 1 then
            phase inputNumbers n
        else
            phase inputNumbers n |> inner (count - 1)

    inner numPhases startingInput
    |> Array.take 8
    |> Array.map string
    |> String.Concat


[<EntryPoint>]
let main argv =
    argv.[1]
    |> File.ReadLines
    |> Seq.head
    |> findSolution (int argv.[0])
    |> printfn "%s"
    0
