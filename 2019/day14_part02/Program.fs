open System
open System.IO
open System.Text.RegularExpressions


module Queue =
    type 'a Queue = Queue of 'a list * 'a list

    let empty = Queue([], [])

    let isEmpty =
        function
        | Queue([], []) -> true
        | _ -> false

    let enqueue a = function
        | Queue(front, back) -> Queue(front, a :: back)

    let dequeue =
        function
        | Queue([], []) -> failwith "Empty queue"
        | Queue(x :: front, tail) -> (x, Queue(front, tail))
        | Queue([], tail) ->
            let front = List.rev tail
            (List.head front, Queue(List.tail front, []))

    let tryDequeue =
        function
        | Queue([], []) -> None
        | q -> Some(dequeue q)

    let ofSeq s = Seq.fold (fun q e -> enqueue e q) empty s

type Rule =
    { Requirements: (int64 * string) list
      Product: int64 * string }

let pattern = Regex "(\d+ \w+)"

let parseRule s =
    let parsePair (s: string) =
        let split = s.Split(' ')
        (int64 split.[0], split.[1])

    let rec inner acc pairs =
        match pairs with
        | [] -> failwithf "Empty rules from line %s" s
        | [ x ] ->
            { Requirements = List.rev acc
              Product = parsePair x }
        | x :: xs -> inner (parsePair x :: acc) xs

    pattern.Matches(s)
    |> Seq.map (fun m -> m.ToString())
    |> List.ofSeq
    |> inner []

let ceilDiv a b =
    (a / b) + if a % b <> 0L then 1L
              else 0L

let rec produceFuel rules (oreRequired: int64) extra needed =
    match Queue.tryDequeue needed with
    | None -> oreRequired, extra
    | Some((requiredCount, requiredChem), needed) ->
        let extraCount =
            extra
            |> Map.tryFind requiredChem
            |> Option.defaultValue 0L

        match requiredChem with
        | "ORE" ->
            let extraConsumed = min requiredCount extraCount
            let quantity = requiredCount - extraConsumed
            let extra = extra |> Map.add requiredChem (extraCount - extraConsumed)
            produceFuel rules (oreRequired + quantity) extra needed
        | _ ->
            let extraUsed = min requiredCount extraCount
            let quantity = requiredCount - extraUsed
            if quantity > 0L then
                let rule = rules |> List.find (fun rule -> (snd rule.Product) = requiredChem)
                let multiplier = ceilDiv quantity (fst rule.Product)
                let extra = extra |> Map.add requiredChem ((fst rule.Product) * multiplier - quantity)
                rule.Requirements
                |> Seq.map (fun (n, s) -> (n * multiplier, s))
                |> Seq.fold (fun q e -> Queue.enqueue e q) needed
                |> produceFuel rules oreRequired extra
            else
                let extra = extra |> Map.add requiredChem (extraCount - extraUsed)
                produceFuel rules oreRequired extra needed

let findSolution rules =
    let targetOre = 1000000000000L

    let produceWithGivenFuel fuelCount =
        produceFuel rules 0L (Map.ofSeq [ ("ORE", targetOre) ]) (Queue.ofSeq [ (fuelCount, "FUEL") ])

    let rec binarySearch lo hi =
        if hi < lo then failwith "Search failed"

        let mid = lo + (hi - lo) / 2L

        let (oreNeeded, extra) = produceWithGivenFuel mid
        let extraOre = Map.tryFind "ORE" extra |> Option.defaultValue 0L

        if oreNeeded = 0L then
            let right = fst <| produceWithGivenFuel (mid + 1L)
            if extraOre > 0L then
                if right > 0L then mid
                else binarySearch (mid + 1L) hi
            else mid
        elif oreNeeded > 0L then
            binarySearch lo (mid - 1L)
        else
            failwithf "b shouldn't be negative"

    let oreForOne = fst <| produceFuel rules 0L Map.empty (Queue.ofSeq [ (1L, "FUEL") ])
    let lo = targetOre / oreForOne
    let hi = 2L * lo
    binarySearch lo hi

[<EntryPoint>]
let main argv =
    argv.[0]
    |> File.ReadLines
    |> Seq.map parseRule
    |> List.ofSeq
    |> findSolution
    |> printfn "%i"
    0
