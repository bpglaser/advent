open System
open System.Text
open System.IO

type Node =
    | Start
    | End
    | Capital of string
    | Lowercase of string

let upper = [ 'A' .. 'Z' ]

let parseNode s =
    let isAllCaps s =
        s
        |> Seq.forall (fun (c: char) -> List.contains c upper)

    match s with
    | "start" -> Start
    | "end" -> End
    | s when isAllCaps s -> Capital s
    | s -> Lowercase s

let parseLine (s: string) =
    match s.Split [| '-' |] with
    | [| left; right |] -> parseNode left, parseNode right
    | _ -> failwithf "malformed line: %s" s

let parse (s: string) =
    let append node entry =
        match entry with
        | Some nodes -> Set.add node nodes |> Some
        | None -> node |> Set.singleton |> Some

    let folder acc (a, b) =
        acc
        |> Map.change a (append b)
        |> Map.change b (append a)

    s.Split(
        [| '\n' |],
        StringSplitOptions.TrimEntries
        ||| StringSplitOptions.RemoveEmptyEntries
    )
    |> Seq.map parseLine
    |> Seq.fold folder Map.empty

let findAllPaths m =
    let findNeighbors pos =
        Map.tryFind pos m |> Option.defaultValue Set.empty

    let rec inner path seen =
        seq {
            match List.tryHead path with
            | None -> failwithf "invalid state: empty path stack"
            | Some End -> yield List.rev path
            | Some head ->
                for neighbor in (findNeighbors head) - seen do
                    match neighbor with
                    | Lowercase _ ->
                        let seen = Set.add neighbor seen
                        yield! inner (neighbor :: path) seen
                    | _ -> yield! inner (neighbor :: path) seen
        }

    inner [ Start ] (Set.singleton Start)

let nodeToString n =
    match n with
    | Start -> "start"
    | End -> "end"
    | Capital s
    | Lowercase s -> s

let join (sep: string) (strings: string list) =
    let sb = new StringBuilder()
    sb.Append(strings.Head) |> ignore

    for s in strings.Tail do
        sb.Append(sep) |> ignore
        sb.Append(s) |> ignore

    sb.ToString()

[<EntryPoint>]
let main args =
    let paths =
        args[0]
        |> File.ReadAllText
        |> parse
        |> findAllPaths
        |> List.ofSeq

    paths
    |> List.map (List.map nodeToString >> join ",")
    |> join "\n"
    |> printfn "%s"

    paths |> List.length |> printfn "%A"

    0
