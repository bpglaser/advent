open System
open System.IO

let parse (s: string) = s.Split(',') |> Array.map int

let scoreMove positions taget =
    positions |> Seq.sumBy (fun i -> abs (i - taget))

let findPosition (positions: int seq) =
    let min, max =
        positions
        |> Seq.fold (fun (min, max) i -> Math.Min(min, i), Math.Max(max, i)) (Int32.MaxValue, 0)

    seq { min..max }
    |> Seq.map (fun i -> i, scoreMove positions i)
    |> Seq.minBy snd

[<EntryPoint>]
let main args =
    let pos, score =
        args[0]
        |> File.ReadLines
        |> Seq.head
        |> parse
        |> findPosition

    printfn "Position: %i, Score: %i" pos score

    0
