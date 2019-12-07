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

type PausedWithOutput =
    { Output: int; Continue: unit -> MachineResult }
and MachineResult =
    | AwaitingInput of (int -> MachineResult)
    | Complete
    | HasOutput of PausedWithOutput

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

let rec runIntCode pc (state: int array) =
    let recur pc = runIntCode pc state

    let get (i, mode) =
        match mode with
        | Position -> state.[i]
        | Immediate -> i

    let instruction = parseInstruction pc state
    let newPC = pc + 1 + instruction.Params.Length

    match instruction.Op with
    | Add ->
        state.[fst instruction.Params.[2]] <- get instruction.Params.[0] + get instruction.Params.[1]
        recur newPC
    | Mul ->
        state.[fst instruction.Params.[2]] <- get instruction.Params.[0] * get instruction.Params.[1]
        recur newPC
    | Input ->
        let continueExec input =
            state.[fst instruction.Params.[0]] <- input
            runIntCode newPC state
        AwaitingInput continueExec
    | Output ->
        let continueExec() = runIntCode newPC state
        HasOutput { Output = (get instruction.Params.[0]); Continue = continueExec }
    | JumpTrue ->
        if get instruction.Params.[0] <> 0 then recur (get instruction.Params.[1])
        else recur newPC
    | JumpFalse ->
        if get instruction.Params.[0] = 0 then recur (get instruction.Params.[1])
        else recur newPC
    | LessThan ->
        if get instruction.Params.[0] < get instruction.Params.[1] then
            state.[fst instruction.Params.[2]] <- 1
        else
            state.[fst instruction.Params.[2]] <- 0
        recur newPC
    | Equals ->
        if get instruction.Params.[0] = get instruction.Params.[1] then
            state.[fst instruction.Params.[2]] <- 1
        else
            state.[fst instruction.Params.[2]] <- 0
        recur newPC
    | Halt -> Complete

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

let setPhases state phases =
    let giveInput value state =
        match state with
        | AwaitingInput func -> func value
        | _ -> failwithf "Invalid state encountered"

    let machines =
        [| for phase in phases ->
            runIntCode 0 (Array.copy state) |> giveInput phase |]

    let mutable outputs = []

    let rec runUntilHalt i input =
        match machines.[i] with
        | AwaitingInput continueFunc ->
            match continueFunc input with
            | AwaitingInput _ -> failwith "Encountered a machine awaiting input when it shouldn't be"
            | HasOutput result ->
                outputs <- if i = 4 then result.Output :: outputs else outputs
                machines.[i] <- result.Continue()
                runUntilHalt ((i + 1) % machines.Length) result.Output
            | Complete -> ()
        | Complete -> ()
        | result -> failwithf "Encountered a machine not awaiting input: %A" result

    runUntilHalt 0 0
    outputs |> List.max

let findSolution state =
    [ 5 .. 9 ]
    |> permutations
    |> Seq.map (fun perm -> (perm, setPhases state perm))
    |> Seq.maxBy snd

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
