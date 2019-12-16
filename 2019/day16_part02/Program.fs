open System
open System.IO

module Seq =
    let rec repeat n s =
        seq {
            if n > 0 then
                yield! s
                yield! repeat (n - 1) s
        }

let parseLine (s: string) =
    s
    |> Seq.map (fun c -> int c - int '0')
    |> Array.ofSeq

let phase inputNumbers =
    let n = Array.length inputNumbers

    let rec inner i s =
        seq {
            yield ((s % 10) + 10) % 10
            if i < n then yield! inner (i + 1) (s - inputNumbers.[i])
        }
    inputNumbers
    |> Array.sum
    |> inner 0
    |> Array.ofSeq

let findSolution numPhases (line: string) =
    let startingInput =
        parseLine line
        |> Seq.repeat 10000
        |> Array.ofSeq

    let offset =
        startingInput
        |> Array.take 7
        |> Array.fold (fun acc i -> acc * 10 + i) 0

    let rec inner count inputNumbers =
        if count = 1 then phase inputNumbers
        else phase inputNumbers |> inner (count - 1)

    startingInput
    |> Array.skip offset
    |> inner numPhases
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
