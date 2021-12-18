open System.IO
open System

module Trie =
    type Branch =
        { children: Map<char, Node>
          count: int }

    and Node =
        | Branch of Branch
        | Leaf of int

    let empty = Leaf 0

    let rec create whole s n =
        match s with
        | [] -> Leaf(n)
        | x :: xs ->
            Branch
                ({ children = Map.add x (create whole xs n) Map.empty
                   count = 1 })

    let rec count (node: Node) =
        match node with
        | Leaf(count) -> count
        | Branch(branch) -> branch.children |> Map.fold (fun acc _ v -> acc + count v) 0

    let insertCount node s n =
        let rec inner node cs =
            match cs, node with
            | [], Leaf(_) -> Leaf(n)
            | cs, Leaf(_) -> create s cs n
            | c :: cs, Branch(branch) ->
                let children =
                    branch.children
                    |> Map.change c (fun entry ->
                           match entry with
                           | Some entry -> inner entry cs |> Some
                           | None -> create s cs n |> Some)

                let count =
                    children
                    |> Seq.map (fun entry -> entry.Value |> count)
                    |> Seq.sum

                { children = children
                  count = count }
                |> Branch
            | _ -> failwith "unreachable"
        inner node (s |> List.ofSeq)

    let insert node s = insertCount node s 1

    let traverse f node =
        let rec inner (acc: char list) node =
            match node with
            | Branch(branch) ->
                let (c, child) = f branch
                inner (c :: acc) child
            | Leaf(count) ->
                (acc
                 |> List.rev
                 |> String.Concat, count)
        inner [] node

let toInt =
    Seq.map (function
        | '0' -> 0
        | '1' -> 1
        | c -> failwithf "unknown char: %c" c)
    >> Seq.fold (fun acc n -> (acc <<< 1) + n) 0

let counts (branch: Trie.Branch) =
    let count c =
        branch.children
        |> Map.tryFind c
        |> Option.map Trie.count
        |> Option.defaultValue 0
    (count '0', count '1')

let oxygen root =
    let traversal (branch: Trie.Branch) =
        let zeroCount, oneCount = counts branch
        if zeroCount > oneCount then '0', branch.children |> Map.find '0' else '1', branch.children |> Map.find '1'
    Trie.traverse traversal root
    |> fst
    |> toInt

let c02 root =
    let traversal (branch: Trie.Branch) =
        let zeroCount, oneCount = counts branch
        if zeroCount = 0 then '1', branch.children |> Map.find '1'
        elif oneCount = 0 then '0', branch.children |> Map.find '0'
        elif zeroCount <= oneCount then '0', branch.children |> Map.find '0'
        else '1', branch.children |> Map.find '1'
    Trie.traverse traversal root
    |> fst
    |> toInt

[<EntryPoint>]
let main argv =
    let lines = argv.[0] |> File.ReadAllLines
    let trie = lines |> Seq.fold Trie.insert Trie.empty
    let a, b = (oxygen trie, c02 trie)
    printfn "%A * %A = %A" a b (a * b)
    0
