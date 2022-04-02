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

let tryFindCorruption chunks =
    chunks
    |> Seq.collect traverse
    |> Seq.choose (fun (l, r) ->
        r
        |> Option.bind (fun r -> if r = getRight l then None else Some r))
    |> Seq.tryHead

let points c =
    match c with
    | ')' -> 3
    | ']' -> 57
    | '}' -> 1197
    | '>' -> 25137
    | _ -> failwithf "tried to point invalid char: %c" c

let score cs =
    cs
    |> Seq.countBy id
    |> Seq.sumBy (fun (c, count) -> points c * count)

let solve input =
    input
    |> Seq.mapi (fun row line ->
        makeContext row line
        |> many1 parseChunk
        |> mustParse)
    |> Seq.choose tryFindCorruption
    |> score

[<EntryPoint>]
let main args =
    args[0]
    |> File.ReadAllLines
    |> Array.map (Array.ofSeq)
    |> solve
    |> printfn "%A"

    0
