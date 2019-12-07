open System
open System.IO

type Opcode =
    | Add
    | Mul
    | Input
    | Output
    | JumpTrue
    | JumpFalse
    | LessThan
    | Equals
    | Halt

type ParameterMode =
    | Position
    | Immediate

type Instruction =
    { Op: Opcode
      Params: (int * ParameterMode) array }

let parseInstruction idx (state: int array) =
    let de = state.[idx] % 100
    let c = (state.[idx] / 100) % 10
    let b = (state.[idx] / 1000) % 10
    let a = (state.[idx] / 10000) % 10

    let modes =
        [| c; b; a |]
        |> Array.map (fun i ->
            match i with
            | 0 -> Position
            | 1 -> Immediate
            | _ -> failwithf "Invalid mode encountered state[%i]=%i" idx state.[idx])

    let (op, numArgs) =
        match de with
        | 1 -> (Add, 3)
        | 2 -> (Mul, 3)
        | 3 -> (Input, 1)
        | 4 -> (Output, 1)
        | 5 -> (JumpTrue, 2)
        | 6 -> (JumpFalse, 2)
        | 7 -> (LessThan, 3)
        | 8 -> (Equals, 3)
        | 99 -> (Halt, 0)
        | _ -> failwithf "Invalid opcode encountered state[%i]=%i" idx state.[idx]

    let parameters =
        [| for i in 1 .. numArgs -> (state.[idx + i], modes.[i - 1]) |]

    { Op = op; Params = parameters }

let promptInput() =
    printf "> "
    Console.ReadLine() |> int

let rec runIntCode readInput writeOutput pc (state: int array)  =
    let recur pc = runIntCode readInput writeOutput pc state

    let get (i, mode) =
        match mode with
        | Position -> state.[i]
        | Immediate -> i

    let instruction = parseInstruction pc state

    match instruction.Op with
    | Add ->
        state.[fst instruction.Params.[2]] <- get instruction.Params.[0] + get instruction.Params.[1]
        recur (pc + 1 + instruction.Params.Length)
    | Mul ->
        state.[fst instruction.Params.[2]] <- get instruction.Params.[0] * get instruction.Params.[1]
        recur (pc + 1 + instruction.Params.Length)
    | Input ->
        state.[fst instruction.Params.[0]] <- readInput()
        recur (pc + 1 + instruction.Params.Length)
    | Output ->
        writeOutput (get instruction.Params.[0])
        recur (pc + 1 + instruction.Params.Length)
    | JumpTrue ->
        if get instruction.Params.[0] <> 0 then recur (get instruction.Params.[1])
        else recur (pc + 1 + instruction.Params.Length)
    | JumpFalse ->
        if get instruction.Params.[0] = 0 then recur (get instruction.Params.[1])
        else recur (pc + 1 + instruction.Params.Length)
    | LessThan ->
        if get instruction.Params.[0] < get instruction.Params.[1] then
            state.[fst instruction.Params.[2]] <- 1
        else
            state.[fst instruction.Params.[2]] <- 0
        recur (pc + 1 + instruction.Params.Length)
    | Equals ->
        if get instruction.Params.[0] = get instruction.Params.[1] then
            state.[fst instruction.Params.[2]] <- 1
        else
            state.[fst instruction.Params.[2]] <- 0
        recur (pc + 1 + instruction.Params.Length)
    | Halt -> state.[0]

let rec inserts x l =
    seq {
        match l with
        | [] -> yield [x]
        | y::rest ->
            yield x::l
            for i in inserts x rest do
              yield y::i
    }

let rec permutations l =
    seq { 
        match l with
        | [] -> yield []
        | x::rest ->
            for p in permutations rest do
              yield! inserts x p
    }

let runEnvironment inputs state =
    let mutable inputs = inputs
    let readInput() =
        let next = List.head inputs
        inputs <- List.tail inputs
        next
    let mutable outputs = []
    let writeOutput x = outputs <- outputs @ [x]
    let state = Array.copy state
    runIntCode readInput writeOutput 0 state |> ignore
    outputs

let findSolution state =
    let rec inner input phases =
        match phases with
        | [] -> failwithf "Reached empty phase list"
        | [phase] -> List.head <| runEnvironment [phase; input] state
        | phase::rest ->
            let output = runEnvironment [phase; input] state
            inner (List.head output) rest

    [ 0 .. 4 ]
    |> permutations
    |> Seq.map (fun perm -> (perm, inner 0 perm))
    |> Seq.maxBy (fun (_, ans) -> ans)

[<EntryPoint>]
let main argv =
    argv.[0]
    |> File.ReadLines
    |> Seq.head
    |> (fun s -> s.Split(','))
    |> Array.map int
    |> findSolution
    |> printfn "%A"
    0
