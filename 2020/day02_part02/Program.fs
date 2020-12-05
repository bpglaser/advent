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
    (entry.Pass.[entry.Lo - 1] = entry.C) <> (entry.Pass.[entry.Hi - 1] = entry.C)

[<EntryPoint>]
let main argv =
    argv.[0]
    |> File.ReadAllLines
    |> Seq.map (parseLine >> isMatch)
    |> Seq.filter id
    |> Seq.length
    |> printfn "%A"
    0