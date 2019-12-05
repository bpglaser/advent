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

let rec runIntCode readInput pc (state: int array)  =
    let recur pc = runIntCode readInput pc state

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
        state.[fst instruction.Params.[0]] <- promptInput()
        recur (pc + 1 + instruction.Params.Length)
    | Output ->
        printfn "%i" (get instruction.Params.[0])
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

[<EntryPoint>]
let main argv =
    argv.[0]
    |> File.ReadLines
    |> Seq.head
    |> (fun s -> s.Split(','))
    |> Array.map int
    |> runIntCode promptInput 0
    |> ignore
    0
