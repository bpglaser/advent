open System
open System.Text.RegularExpressions
open System.IO

let rule = Regex("(\d+)-(\d+) (\w): (\w+)")

type Entry =
    { Lo: int
      Hi: int
      C: char
      Pass: string }

let parseLine line =
    let regexMatch = rule.Match line
    let groups = regexMatch.Groups
    { Lo = int groups.[1].Value
      Hi = int groups.[2].Value
      C = groups.[3].Value.[0]
      Pass = groups.[4].Value }

let isMatch entry =
    let counter acc c =
        match Map.tryFind c acc with
        | Some i -> Map.add c (i + 1) acc
        | None -> Map.add c 1 acc
    let count = 
        entry.Pass
        |> Seq.fold counter Map.empty
        |> Map.tryFind entry.C
        |> Option.defaultValue 0
    (entry.Lo <= count) && (count <= entry.Hi)

[<EntryPoint>]
let main argv =
    argv.[0]
    |> File.ReadAllLines
    |> Seq.map (parseLine >> isMatch)
    |> Seq.filter id
    |> Seq.length
    |> printfn "%A"
    0