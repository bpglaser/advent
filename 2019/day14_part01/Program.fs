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
    { Requirements: (int * string) list
      Product: int * string }

let pattern = Regex "(\d+ \w+)"

let parseRule s =
    let parsePair (s: string) =
        let split = s.Split(' ')
        (int split.[0], split.[1])

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
    (a / b) + if a % b <> 0 then 1
              else 0

let findSolution (rules: Rule list) =
    let rec inner oreRequired extra needed =
        match Queue.tryDequeue needed with
        | None -> oreRequired
        | Some((requiredCount, requiredChem), needed) ->
            let extraCount =
                extra
                |> Map.tryFind requiredChem
                |> Option.defaultValue 0

            match requiredChem with
            | "ORE" ->
                let extraConsumed = min requiredCount extraCount
                let quantity = requiredCount - extraConsumed
                let extra = extra |> Map.add requiredChem (extraCount - extraConsumed)
                inner (oreRequired + quantity) extra needed
            | _ ->
                let extraUsed = min requiredCount extraCount
                let quantity = requiredCount - extraUsed
                if quantity > 0 then
                    let rule = rules |> List.find (fun rule -> (snd rule.Product) = requiredChem)
                    let multiplier = ceilDiv quantity (fst rule.Product)
                    let extra = extra |> Map.add requiredChem ((fst rule.Product) * multiplier - quantity)
                    rule.Requirements
                    |> Seq.map (fun (n, s) -> (n * multiplier, s))
                    |> Seq.fold (fun q e -> Queue.enqueue e q) needed
                    |> inner oreRequired extra
                else
                    let extra = extra |> Map.add requiredChem (extraCount - extraUsed)
                    inner oreRequired extra needed
    inner 0 Map.empty (Queue.ofSeq [ (1, "FUEL") ])

[<EntryPoint>]
let main argv =
    argv.[0]
    |> File.ReadLines
    |> Seq.map parseRule
    |> List.ofSeq
    |> findSolution
    |> printfn "%i"
    0
