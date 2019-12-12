open System
open System.IO
open System.Text.RegularExpressions

module Seq =
    let rec all pred s =
        if Seq.isEmpty s then true
        elif s
             |> Seq.head
             |> pred
        then all pred (Seq.tail s)
        else false

type Moon =
    { Position: int array
      Velocity: int array }

let copyMoon moon =
    { Position = Array.copy moon.Position
      Velocity = Array.copy moon.Velocity }

let parseInput lines =
    let regex = Regex "<x=(-?\d+), y=(-?\d+), z=(-?\d+)>"
    lines
    |> Seq.map (fun line ->
        regex.Match(line).Groups
        |> Seq.skip 1
        |> Seq.map (fun group -> int group.Value)
        |> Array.ofSeq
        |> (fun pos ->
        { Position = pos
          Velocity = Array.create pos.Length 0 }))
    |> List.ofSeq

let calculateVelocity a b =
    [| for i in 0 .. a.Position.Length - 1 do
        yield a.Velocity.[i] + if a.Position.[i] < b.Position.[i] then 1
                               elif a.Position.[i] > b.Position.[i] then -1
                               else 0 |]

let applyGravity moons =
    let moons =
        moons
        |> Seq.map copyMoon
        |> Array.ofSeq
    for i in 0 .. moons.Length - 1 do
        for j in i + 1 .. moons.Length - 1 do
            moons.[i] <- { moons.[i] with Velocity = calculateVelocity moons.[i] moons.[j] }
            moons.[j] <- { moons.[j] with Velocity = calculateVelocity moons.[j] moons.[i] }
    List.ofArray moons

let addArray a b = Array.zip a b |> Array.map (fun (i, j) -> i + j)

let updateVelocity moons =
    moons |> List.map (fun moon -> { moon with Position = (addArray moon.Position moon.Velocity) })

let step = applyGravity >> updateVelocity

let getState i moons =
    [| for moon in moons do
        yield moon.Position.[i]
        yield moon.Velocity.[i] |]

let updateSeen steps moons i (seen, answer) =
    match answer with
    | Some _ -> (seen, answer)
    | None ->
        let state = getState i moons
        if Set.contains state seen then (Set.add state seen, Some steps)
        else (Set.add state seen, None)

let rec euclidsAlgorithm a b =
    if b = bigint.Zero then a
    else euclidsAlgorithm b (bigint.Remainder(a, b))

let leastCommonMultiple a b = (abs (a * b)) / (euclidsAlgorithm a b)

let findSolution (moons: Moon list) =
    let rec inner steps seens moons =
        if seens |> Seq.all (snd >> Option.isSome) then
            seens |> List.map (snd >> Option.get)
        else
            let steps = steps + 1
            let moons = step moons
            let seens = List.mapi (updateSeen steps moons) seens
            inner steps seens moons

    inner 0 [ for i in 0 .. 2 -> (set [ getState i moons ], None) ] moons
    |> List.map bigint
    |> List.reduce leastCommonMultiple


[<EntryPoint>]
let main argv =
    argv.[0]
    |> File.ReadLines
    |> parseInput
    |> findSolution
    |> printfn "%A"
    0
