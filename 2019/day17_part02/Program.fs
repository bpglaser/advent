open System.IO
open Intcode

module Seq =
    let rec all pred s =
        if s |> Seq.isEmpty then true
        elif s |> Seq.head |> pred then s |> Seq.tail |> all pred
        else false

type Grid = Map<int * int, char>

let createGrid result =
    let rec inner buf result =
        match result with
        | AwaitingInput cont -> failwithf "Unexpected read"
        | Complete -> [ buf ]
        | HasOutput output ->
            let c =
                output.Output
                |> int
                |> char
            match c with
            | '\n' -> (List.rev buf) :: inner [] (output.Continue())
            | _ -> inner (c :: buf) (output.Continue())

    result
    |> inner []
    |> Seq.mapi (fun y row -> row |> Seq.mapi (fun x c -> ((x, y), c)))
    |> Seq.collect id
    |> Map.ofSeq

let isIntersection (grid: Grid) (x, y) =
    let matches =
        seq {
            (x - 1, y)
            (x + 1, y)
            (x, y - 1)
            (x, y + 1)
        }
        |> Seq.choose (fun pos -> Map.tryFind pos grid)
        |> Seq.filter (fun c -> c = '#')
        |> Seq.length
    matches = 4

let findScaffoldIntersections (grid: Grid) =
    grid
    |> Seq.choose (fun entry ->
        if entry.Value = '#' then Some entry.Key
        else None)
    |> Seq.filter (isIntersection grid)

let printGrid (grid: Grid) =
    let maxX =
        grid
        |> Seq.map (fun entry -> fst entry.Key)
        |> Seq.max

    let maxY =
        grid
        |> Seq.map (fun entry -> snd entry.Key)
        |> Seq.max

    for y in 0 .. maxY do
        for x in 0 .. maxX do
            printf "%c" (Map.find (x, y) grid)
        printfn ""

type Offsets =
    { Left: (int * int) * char
      Right: (int * int) * char
      Forward: (int * int) * char
      Back: (int * int) * char }

let getOffsets (x, y) facing =
    match facing with
    | '<' ->
        { Left = ((x, y + 1), 'v')
          Right = ((x, y - 1), '^')
          Forward = ((x - 1, y), '<')
          Back = ((x + 1, y), '>') }
    | '>' ->
        { Left = ((x, y - 1), '^')
          Right = ((x, y + 1), 'v')
          Forward = ((x + 1, y), '>')
          Back = ((x - 1, y), '<') }
    | '^' ->
        { Left = ((x - 1, y), '<')
          Right = ((x + 1, y), '>')
          Forward = ((x, y - 1), '^')
          Back = ((x, y + 1), 'v') }
    | 'v' ->
        { Left = ((x + 1, y), '>')
          Right = ((x - 1, y), '<')
          Forward = ((x, y + 1), 'v')
          Back = ((x, y - 1), '^') }
    | _ -> failwithf "Invalid facing %c" facing

let rec walk acc (pos, facing) (grid: Grid) =
    let lookup (pos, _) =
        grid |> Map.tryFind pos |> Option.defaultValue '.'

    let offsets = getOffsets pos facing

    match lookup offsets.Forward with
    | '#' -> walk (acc + 1) offsets.Forward grid
    | '.' ->
        if lookup offsets.Left = '#' then
            (string acc) :: "L" :: walk 1 offsets.Left grid
        elif lookup offsets.Right = '#' then
            (string acc) :: "R" :: walk 1 offsets.Right grid
        else
            [string acc]
    | tile -> failwithf "Invalid tile encountered: %c" tile

let findPath (grid: Grid) =
    let start =
        grid
        |> Seq.map (fun entry -> (entry.Key, entry.Value))
        |> Seq.find (fun (_, tile) -> tile = '^' || tile = '>' || tile = 'v' || tile = '<')

    let cleanGrid =
        grid
        |> Map.map (fun k v -> if (k, v) = start then '#' else v)

    walk 0 start cleanGrid
    |> List.filter ((<>) "0")

let createInstruction sub = sub |> String.concat ","

let largestNonOverlappingSubstring values =
    let values = Array.ofSeq values
    let substring (i, length) =
        values |> Array.skip i |> Array.truncate length

    let rec buildCounts map sub =
        let count = Map.tryFind sub map |> Option.defaultValue 0
        Map.add sub (count + 1) map

    seq {
        for i in 0 .. values.Length - 1 do
            for j in i + 1 .. values.Length - 1 do
                let length = j - i
                for k in j + 1 .. values.Length - 1 do
                    let sub1 = substring (i, length)
                    let sub2 = substring (k, length)
                    if sub1 = sub2 then
                        yield sub1
    }
    |> Seq.fold buildCounts Map.empty
    |> Map.toSeq
    |> Seq.filter (fun (sub, count) -> sub |> createInstruction |> String.length <= 20 && count > 1)
    |> List.ofSeq
    |> List.maxBy (fun (sub, _) -> sub.Length)
    |> fst
    |> List.ofArray

type Instruction =
    | Sequence of string list
    | Mapped of string * string list

let isMapped =
    function
    | Sequence _ -> false
    | Mapped _ -> true

let partitionInstruction instruction replacement =
    let i =
        seq { 0 .. List.length instruction - 1}
        |> Seq.find (fun i ->
            Seq.skip i instruction
            |> Seq.zip replacement
            |> Seq.all (fun (a, b) -> a = b))
    [Sequence (List.take i instruction); Mapped ]

let rec replaceLargest instruction =
    match instruction with
    | Mapped _ -> [instruction]
    | Sequence values ->
        let largest = largestNonOverlappingSubstring values
        partitionInstruction values largest
        |> replaceLargest

let createMappedSequence path =
    let rec inner instructions =
        if Seq.all isMapped instructions then
            instructions
        else
            instructions
            |> List.collect replaceLargest
            |> inner
    inner [Sequence path]

let findSolution line =
    let grid =
        line
        |> createState
        |> runIntCode
        |> createGrid

    printGrid grid

    let path = findPath grid
    printfn "%A" path
    let largest = largestNonOverlappingSubstring path
    printfn "%A" largest

    grid
    |> findScaffoldIntersections
    |> Seq.sumBy (fun (a, b) -> a * b)

[<EntryPoint>]
let main argv =
    argv.[0]
    |> File.ReadLines
    |> Seq.head
    |> findSolution
    |> printfn "%A"
    0
