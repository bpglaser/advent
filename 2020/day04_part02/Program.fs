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

let validIntRange lo hi s = 
    try
        let i = int s
        lo <= i && i <= hi
    with
    | _ -> false

let validHeight s =
    let pattern = Regex("(\d+)(cm|in)")
    let patternMatch = pattern.Match(s)
    if patternMatch.Success then
        let groups = patternMatch.Groups
        match groups.[2].Value with
        | "cm" -> groups.[1].Value |> validIntRange 150 193
        | "in" -> groups.[1].Value |> validIntRange 59 76
        | _ -> failwithf "Invalid height [%s, %A]" s groups
    else false

let validHair s = Regex("^#[0-9a-f]{6}$").IsMatch(s)

let validEye s = Regex("amb|blu|brn|gry|grn|hzl|oth").IsMatch(s)

let validPassportID s = Regex("^\d{9}$").IsMatch(s)

let fieldValidators =
    [
        ("byr", validIntRange 1920 2002)
        ("iyr", validIntRange 2010 2020)
        ("eyr", validIntRange 2020 2030)
        ("hgt", validHeight)
        ("hcl", validHair)
        ("ecl", validEye)
        ("pid", validPassportID)
    ]

let isBlankLine (line: string) = line.Length = 0

let rec splitPassports lines =
    match lines with
    | [] -> []
    | lines ->
        let group = List.takeWhile (isBlankLine >> not) lines
        let len = List.length group
        let rest = List.skip len lines
        let rest = List.skipWhile isBlankLine rest
        group :: splitPassports rest

let getPassport (lines: string seq) : Passport =
    let toTuple (words: string array) =
        (words.[0], words.[1])

    lines
    |> Seq.collect (fun line -> line.Split() |> Seq.map (fun entry -> entry.Split(':') |> toTuple))
    |> Map.ofSeq

let isValidPassport passport =
    let validFields =
        fieldValidators
        |> Seq.map (fun (key, pred) ->
            passport
            |> Map.tryFind key
            |> Option.map (fun value -> key, pred value)
            |> Option.defaultValue (key, false))
        |> Seq.filter snd
        |> Seq.length
    validFields = Seq.length fieldValidators


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