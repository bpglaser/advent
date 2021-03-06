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
    | AdjustRB
    | Halt

type ParameterMode =
    | Position
    | Immediate
    | Relative

type Instruction =
    { Op: Opcode
      Params: (bigint * ParameterMode) array }

type PausedWithOutput =
    { Output: bigint
      Continue: unit -> MachineResult }

and MachineResult =
    | AwaitingInput of (bigint -> MachineResult)
    | Complete
    | HasOutput of PausedWithOutput

type MachineState =
    { PC: bigint
      RB: bigint
      State: Map<bigint, bigint> }
    member this.Item
        with get (index: bigint) =
            this.State
            |> Map.tryFind index
            |> Option.defaultValue bigint.Zero

let updateEntry index value state =
    let newState = Map.add index value state.State
    { state with State = newState }

let (%) (numerator: bigint) (divisor: int) = bigint.Remainder(numerator, bigint divisor)

let parseInstruction (state: MachineState) =
    let idx = state.PC
    let de = state.[idx] % 100
    let c = (state.[idx] / 100I) % 10
    let b = (state.[idx] / 1000I) % 10
    let a = (state.[idx] / 10000I) % 10

    let modes =
        [| c; b; a |]
        |> Array.map (fun i ->
            match (int i) with
            | 0 -> Position
            | 1 -> Immediate
            | 2 -> Relative
            | _ -> failwithf "Invalid mode encountered state[%A]=%A" idx state.[idx])

    let (op, numArgs) =
        match int de with
        | 1 -> (Add, 3)
        | 2 -> (Mul, 3)
        | 3 -> (Input, 1)
        | 4 -> (Output, 1)
        | 5 -> (JumpTrue, 2)
        | 6 -> (JumpFalse, 2)
        | 7 -> (LessThan, 3)
        | 8 -> (Equals, 3)
        | 9 -> (AdjustRB, 1)
        | 99 -> (Halt, 0)
        | _ -> failwithf "Invalid opcode encountered state[%A]=%A" idx state.[idx]

    let parameters =
        [| for i in 1 .. numArgs -> (state.[idx + bigint i], modes.[i - 1]) |]

    { Op = op
      Params = parameters }

let rec runIntCode (state: MachineState) =
    let get (i: bigint, mode) =
        match mode with
        | Position -> state.[i]
        | Immediate -> i
        | Relative -> state.[i + state.RB]

    let set (i, mode) value =
        match mode with
        | Position -> updateEntry i value state
        | Immediate -> failwithf "tried to set with an immediate"
        | Relative -> updateEntry (i + state.RB) value state

    let instruction = parseInstruction state
    let newPC = state.PC + 1I + bigint instruction.Params.Length

    match instruction.Op with
    | Add ->
        let index = instruction.Params.[2]
        let value = get instruction.Params.[0] + get instruction.Params.[1]
        let state = set index value
        runIntCode { state with PC = newPC }
    | Mul ->
        let index = instruction.Params.[2]
        let value = get instruction.Params.[0] * get instruction.Params.[1]
        let state = set index value
        runIntCode { state with PC = newPC }
    | Input ->
        let continueExec input =
            let index = instruction.Params.[0]
            let state = set index input
            runIntCode { state with PC = newPC }
        AwaitingInput continueExec
    | Output ->
        let continueExec() = runIntCode { state with PC = newPC }
        HasOutput
            { Output = (get instruction.Params.[0])
              Continue = continueExec }
    | JumpTrue ->
        if get instruction.Params.[0] <> 0I then runIntCode { state with PC = (get instruction.Params.[1]) }
        else runIntCode { state with PC = newPC }
    | JumpFalse ->
        if get instruction.Params.[0] = 0I then runIntCode { state with PC = (get instruction.Params.[1]) }
        else runIntCode { state with PC = newPC }
    | LessThan ->
        let index = instruction.Params.[2]
        if get instruction.Params.[0] < get instruction.Params.[1] then
            let state = set index 1I
            runIntCode { state with PC = newPC }
        else
            let state = set index 0I
            runIntCode { state with PC = newPC }
    | Equals ->
        let index = instruction.Params.[2]
        if get instruction.Params.[0] = get instruction.Params.[1] then
            let state = set index 1I
            runIntCode { state with PC = newPC }
        else
            let state = set index 0I
            runIntCode { state with PC = newPC }
    | AdjustRB ->
        let offset = get instruction.Params.[0]
        runIntCode
            { state with
                  PC = newPC
                  RB = (state.RB + offset) }
    | Halt -> Complete

type Tile =
    | Empty
    | Wall
    | Block
    | Paddle
    | Ball

let tile tileID =
    if tileID = 0I then Empty
    elif tileID = 1I then Wall
    elif tileID = 2I then Block
    elif tileID = 3I then Paddle
    elif tileID = 4I then Ball
    else failwithf "Invalid tileID encountered %A" tileID

let createState (line: string) =
    let state =
        line.Split(',')
        |> Seq.mapi (fun i s -> (bigint i, bigint.Parse s))
        |> Map.ofSeq
    { PC = 0I
      RB = 0I
      State = state }

let runOutput state =
    let rec inner result =
        seq {
            match result with
            | AwaitingInput _ -> failwith "Awaiting input"
            | Complete -> ()
            | HasOutput output ->
                yield output.Output
                yield! output.Continue() |> inner
        }
    state
    |> runIntCode
    |> inner

let findSolution state =
    let folder grid output =
        match output with
        | [| x; y; tileID |] -> Map.add (x, y) (tile tileID) grid
        | _ -> failwithf "Output has incorrect size: %A" output

    state
    |> runOutput
    |> Seq.chunkBySize 3
    |> Seq.fold folder Map.empty
    |> Seq.filter (fun entry -> entry.Value = Block)
    |> Seq.length

[<EntryPoint>]
let main argv =
    argv.[0]
    |> File.ReadLines
    |> Seq.head
    |> createState
    |> findSolution
    |> printfn "%A"
    0
