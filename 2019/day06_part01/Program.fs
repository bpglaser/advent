open System.IO

let parseRelationship (s: string) =
    let split = s.Trim().Split(')')
    if split.Length = 2 then (split.[0], split.[1])
    else failwithf "Invalid entry found: %s" s

let rec buildMap map relationships =
    match relationships with
    | (key, value) :: tail ->
        match Map.tryFind key map with
        | Some connections ->
            let map = Map.add key (value :: connections) map
            buildMap map tail
        | None ->
            let map = Map.add key [value] map
            buildMap map tail
    | [] -> map

let parseLines (lines: string seq) =
    lines
    |> Seq.map parseRelationship
    |> List.ofSeq
    |> buildMap Map.empty

let rec sumOrbits map depth node =
    match Map.tryFind node map with
    | Some children when List.isEmpty children -> depth
    | Some children -> depth + (children |> List.sumBy (sumOrbits map (depth + 1)))
    | None -> depth

let solve lines = lines |> parseLines |> fun map -> sumOrbits map 0 "COM"

[<EntryPoint>]
let main argv =
    argv.[0]
    |> File.ReadLines
    |> solve
    |> printfn "%A"
    0
