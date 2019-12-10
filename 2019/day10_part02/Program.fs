open System
open System.IO

type Grid = char array array

let hashIndicies (grid: Grid) =
    seq {
        for y in 0 .. grid.Length - 1 do
            for x in 0 .. grid.[0].Length - 1 -> (x, y)
    }
    |> Seq.filter (fun (x, y) -> grid.[y].[x] = '#')

let rec euclidsAlgorithm a b =
    match (a, b) with
    | (a, 0) -> a
    | (a, b) -> euclidsAlgorithm b (a % b)

let euclideanDistance (x1, y1) (x2, y2) =
    let dx = float (x1 - x2)
    let dy = float (y1 - y2)
    sqrt (dx ** 2.0 + dy ** 2.0)

let buildGrid lines =
    lines
    |> Seq.map Array.ofSeq
    |> Array.ofSeq

let getAngleRatio startPos endPos =
    let (x, y) = startPos
    let (x1, y1) = endPos
    let (dx, dy) = (x1 - x, y1 - y)
    let gcd = abs (euclidsAlgorithm dx dy)
    (dx / gcd, dy / gcd)

let findAngle (dx, dy) =
    let (dx, dy) = (float dx, float dy)
    let angle = atan2 dy dx
    if angle < 0.0 then 2.0 * Math.PI + angle
    else angle

let countAsteroids (grid: Grid) start =
    grid
    |> hashIndicies
    |> Seq.filter ((<>) start)
    |> Seq.map (getAngleRatio start)
    |> Set.ofSeq
    |> Set.count

let findStart (grid: Grid) =
    let peek =
        grid
        |> hashIndicies
        |> Seq.maxBy (fun pos -> countAsteroids grid pos)
    peek

let rotate asteroids =
    let i = List.findIndex (fun (ratio, _) -> ratio = (0, -1)) asteroids
    List.skip i asteroids @ List.take i asteroids

let findSolution (grid: Grid) start =
    let rec eliminate (asteroids: list<(int * int) * list<int * int>>) =
        seq {
            match asteroids with
            | (ratio, x :: xs) :: rows ->
                yield x
                if List.isEmpty xs then
                    yield! eliminate rows
                else
                    yield! eliminate (rows @ [(ratio, xs)])
            | (ratio, []) :: _ -> failwithf "Invalid empty row encountered for ratio=%A" ratio
            | [] -> ()
        }

    let asteroids =
        grid
        |> hashIndicies
        |> Seq.filter ((<>) start)
        |> List.ofSeq
        |> List.groupBy (getAngleRatio start)
        |> List.map (fun (ratio, items) -> (ratio, List.sortBy (euclideanDistance start) items))
        |> List.sortBy (fun (ratio, _) -> findAngle ratio)
        |> rotate
    let (ansx, ansy) = asteroids |> eliminate |> Seq.item (200 - 1)
    ansx * 100 + ansy

[<EntryPoint>]
let main argv =
    argv.[0]
    |> File.ReadLines
    |> buildGrid
    |> (fun grid -> (grid, findStart grid))
    |> (fun (grid, start) -> findSolution grid start)
    |> printfn "%i"
    0
