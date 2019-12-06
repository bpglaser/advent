open System.IO

module List =
    let filterMap func =
        List.map func >> List.filter Option.isSome >> List.map Option.get

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

let findPath map src dest =
    let rec inner curr =
        if curr = dest then
            Some []
        else 
            match Map.tryFind curr map with
            | Some children -> 
                let path = children |> List.filterMap inner |> List.tryHead
                match path with
                | Some path -> Some (curr :: path)
                | None -> None
            | None -> None
    inner src

let countTransfers map src dest =
    let a = findPath map "COM" src
    let b = findPath map "COM" dest
    match a, b with
    | Some a, Some b ->
        let commonNodes = Set.intersect (set a) (set b)
        let ai = List.tryFindIndexBack (fun node -> Set.contains node commonNodes) a
        let bi = List.tryFindIndexBack (fun node -> Set.contains node commonNodes) b
        match ai, bi with
        | Some ai, Some bi ->
            Some <| List.length a - ai + List.length b - bi - 2
        | _ -> None
    | _ -> None

let solve lines =
    lines |> parseLines |> fun map -> countTransfers map "YOU" "SAN"

[<EntryPoint>]
let main argv =
    argv.[0]
    |> File.ReadLines
    |> solve
    |> fun ans ->
        match ans with
        | Some ans -> printfn "%i" ans
        | None -> printfn "No path found"
    0
