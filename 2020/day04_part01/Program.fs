open System
open System.IO
open System.Text.RegularExpressions

module Seq =
    let rec all pred s =
        if Seq.isEmpty s then
            true
        elif s |> Seq.head |> pred then
            s |> Seq.skip 1 |> all pred
        else
            false

type Passport = Map<string, string>

let requiredFields =
    ["byr"; "iyr"; "eyr"; "hgt"; "hcl"; "ecl"; "pid"]

let optionalFields =
    ["cid"]

let isBlankLine (line: string) = line.Length = 0

let rec splitPassports lines =
    match lines with
    | [] -> []
    | lines ->
        let group = List.takeWhile (isBlankLine >> not) lines
        let len = List.length group
        let rest = List.skip (len) lines
        let rest = List.skipWhile isBlankLine rest
        group :: splitPassports rest

let getPassport (lines: string seq) : Passport =
    let toTuple (regexMatch: Match) =
        (regexMatch.Groups.[1].Value, regexMatch.Groups.[2].Value)

    let pattern = Regex("(\S+):(\S+)")

    lines
    |> Seq.collect (pattern.Matches >> (Seq.map toTuple))
    |> Map.ofSeq

let isValidPassport (passport: Passport) =
    requiredFields
    |> Seq.all (fun field -> Map.containsKey field passport)

[<EntryPoint>]
let main argv =
    argv.[0]
    |> File.ReadAllLines
    |> List.ofArray
    |> splitPassports
    |> List.map (getPassport >> isValidPassport)
    |> List.filter id
    |> List.length
    |> printfn "%A"
    0