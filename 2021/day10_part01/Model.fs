module Model

type Chunk =
    { Left: char
      Right: char option
      Children: Chunk list }

let getRight c =
    match c with
    | '(' -> ')'
    | '[' -> ']'
    | '{' -> '}'
    | '<' -> '>'
    | _ -> failwithf "no right side for %c" c

let rec traverse chunk =
    seq {
        for child in chunk.Children do
            yield! traverse child

        yield (chunk.Left, chunk.Right)
    }

let anyUnfinished chunks =
    chunks
    |> Seq.collect traverse
    |> Seq.tryFind (fun (_, r) -> Option.isNone r)
    |> Option.isSome
