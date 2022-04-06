module Program

open System
open System.IO

let seqGrid =
    Seq.mapi (fun y row -> row |> Seq.mapi (fun x i -> ((x, y), i)))
    >> Seq.collect id

let mapGrid f =
    Array.map (Array.map f)

let mapGridi f =
    Array.mapi (fun y row -> row |> Array.mapi (fun x i -> f (x, y) i))

let parse (s: string) =
    let opts =
        StringSplitOptions.RemoveEmptyEntries
        ||| StringSplitOptions.TrimEntries
    let int c = int c - int '0'

    s.Split([| '\n' |], opts)
    |> Array.map (Seq.map int >> Array.ofSeq)

let incrementAll n = mapGrid (fun i -> i + n)

let singleFlash (x, y) grid =
    let offsets = [ -1 .. 1 ]
    let toUpdate =
        Seq.allPairs offsets offsets
        |> Seq.filter ((<>) (0, 0))
        |> Seq.map (fun (dx, dy) -> (x + dx, y + dy))
        |> Set.ofSeq
    let inc pos n =
        if Set.contains pos toUpdate then n + 1
        else n
    mapGridi inc grid

let rec flash seen grid =
    // Try to find a position that is ready to flash.
    let candidate =
        grid
        |> seqGrid
        |> Seq.tryFind (fun (pos, i) -> i > 9 && not <| Set.contains pos seen)
    match candidate with
    | None -> seen, grid
    | Some (pos, _) ->
        flash (Set.add pos seen) (singleFlash pos grid)

let zeroFlashed (points, grid) =
    let conditionalZero pos n =
        if Set.contains pos points then 0 else n

    Set.count points, mapGridi conditionalZero grid

let step = incrementAll 1 >> flash Set.empty >> zeroFlashed

let solve count s =
    let rec inner i totalFlashes grid =
        if i = 0 then totalFlashes
        else
            let flashes, newGrid = step grid
            inner (i - 1) (totalFlashes + flashes) newGrid
    inner count 0 (parse s)

[<EntryPoint>]
let main args =
    let count = int args[0]
    let path = args[1]
    path
    |> File.ReadAllText
    |> solve count
    |> printfn "%A"

    0
