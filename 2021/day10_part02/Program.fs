module Program

open System.IO

open Model
open Parsing

let parseStart = char '(' <|> char '[' <|> char '{' <|> char '<'

let parseEnd = char ')' <|> char ']' <|> char '}' <|> char '>'

let makeChunk ((l, children), r) =
    { Left = l
      Right = r
      Children = children }

let rec parseChunk input =
    input
    |> (parseStart .>>. many parseChunk .>>. opt parseEnd
        |>> makeChunk)

let makeContext row line =
    { Column = 1
      Row = row + 1
      Chars = List.ofArray line }

let parseRoot row line =
    makeContext row line
    |> many1 parseChunk
    |> mustParse

let tryFindMissing (chunks: Chunk seq) =
    let folder acc (l, r) =
        match acc with
        | Error e -> Error e
        | Ok missing ->
            let expected = getRight l

            match r with
            | Some r ->
                if r = expected then
                    Ok missing
                else
                    Error "missing char"
            | None -> Ok(expected :: missing)

    chunks
    |> Seq.collect traverse
    |> Seq.fold folder (Ok [])
    |> Result.map List.rev

let point c =
    match c with
    | ')' -> 1L
    | ']' -> 2L
    | '}' -> 3L
    | '>' -> 4L
    | _ -> failwithf "tried to score unknown char: %c" c

let score res : int64 =
    let hash n c = n * 5L + (point c)

    match res with
    | Ok missing -> missing |> Seq.fold hash 0L
    | Error _ -> 0

let solve input =
    let scores =
        input
        |> Seq.mapi parseRoot
        |> Seq.map (tryFindMissing >> score)
        |> Seq.filter (fun i -> i > 0)
        |> Array.ofSeq

    Array.sortInPlace scores
    scores[scores.Length / 2]

[<EntryPoint>]
let main args =
    args[0]
    |> File.ReadAllLines
    |> Array.map (Array.ofSeq)
    |> solve
    |> printfn "%d"

    0
