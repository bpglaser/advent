open System
open System.IO
open System.Text.RegularExpressions

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

let calculateEnergy moon = (moon.Position |> Seq.sumBy abs) * (moon.Velocity |> Seq.sumBy abs)

let findSolution numSteps moons =
    [ 1 .. numSteps ]
    |> List.fold (fun state _ -> step state) moons
    |> List.sumBy calculateEnergy


[<EntryPoint>]
let main argv =
    argv.[1]
    |> File.ReadLines
    |> parseInput
    |> findSolution (int argv.[0])
    |> printfn "%A"
    0
