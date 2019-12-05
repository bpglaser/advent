open System
open System.IO

type Opcode =
    | Add
    | Mul
    | Input
    | Output
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
        | 99 -> (Halt, 0)
        | _ -> failwithf "Invalid opcode encountered state[%i]=%i" idx state.[idx]

    let parameters =
        [| for i in 1 .. numArgs -> (state.[idx + i], modes.[i - 1]) |]

    { Op = op; Params = parameters }

let promptInput() =
    printf "> "
    Console.ReadLine() |> int

let rec runIntCode pc (state: int array) =
    let get (i, mode) =
        match mode with
        | Position -> state.[i]
        | Immediate -> i

    let instruction = parseInstruction pc state

    match instruction.Op with
    | Add ->
        state.[fst instruction.Params.[2]] <- get instruction.Params.[0] + get instruction.Params.[1]
        runIntCode (pc + 4) state
    | Mul ->
        state.[fst instruction.Params.[2]] <- get instruction.Params.[0] * get instruction.Params.[1]
        runIntCode (pc + 4) state
    | Input ->
        state.[fst instruction.Params.[0]] <- promptInput()
        runIntCode (pc + 2) state
    | Output ->
        printfn "%i" (get instruction.Params.[0])
        runIntCode (pc + 2) state
    | Halt -> state.[0]

[<EntryPoint>]
let main argv =
    argv.[0]
    |> File.ReadLines
    |> Seq.head
    |> (fun s -> s.Split(','))
    |> Array.map int
    |> runIntCode 0
    |> ignore
    0
