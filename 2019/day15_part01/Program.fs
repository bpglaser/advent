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

type MachineState =
    { PC: bigint
      RB: bigint
      State: Map<bigint, bigint> }
    member this.Item
        with get (index: bigint) =
            this.State
            |> Map.tryFind index
            |> Option.defaultValue bigint.Zero

type PausedWithOutput =
    { PrevState: MachineState
      Output: bigint
      Continue: unit -> MachineResult }

and MachineResult =
    | AwaitingInput of MachineState * (bigint -> MachineResult)
    | Complete of MachineState
    | HasOutput of PausedWithOutput

let updateEntry index value state =
    let newState = Map.add index value state.State
    { state with State = newState }

let (%) (numerator: bigint) (divisor: int) = bigint.Remainder(numerator, bigint divisor)

let parseInstruction (state: MachineState) =
    let idx = state.PC
    let de = state.[idx] % 100
    let c = (state.[idx] / bigint 100) % 10
    let b = (state.[idx] / bigint 1000) % 10
    let a = (state.[idx] / bigint 10000) % 10

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
    let newPC = state.PC + bigint 1 + bigint instruction.Params.Length

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
        AwaitingInput(state, continueExec)
    | Output ->
        let continueExec() = runIntCode { state with PC = newPC }
        HasOutput
            { PrevState = state
              Output = (get instruction.Params.[0])
              Continue = continueExec }
    | JumpTrue ->
        if get instruction.Params.[0] <> bigint.Zero then runIntCode { state with PC = (get instruction.Params.[1]) }
        else runIntCode { state with PC = newPC }
    | JumpFalse ->
        if get instruction.Params.[0] = bigint.Zero then runIntCode { state with PC = (get instruction.Params.[1]) }
        else runIntCode { state with PC = newPC }
    | LessThan ->
        let index = instruction.Params.[2]
        if get instruction.Params.[0] < get instruction.Params.[1] then
            let state = set index bigint.One
            runIntCode { state with PC = newPC }
        else
            let state = set index bigint.Zero
            runIntCode { state with PC = newPC }
    | Equals ->
        let index = instruction.Params.[2]
        if get instruction.Params.[0] = get instruction.Params.[1] then
            let state = set index bigint.One
            runIntCode { state with PC = newPC }
        else
            let state = set index bigint.Zero
            runIntCode { state with PC = newPC }
    | AdjustRB ->
        let offset = get instruction.Params.[0]
        runIntCode
            { state with
                  PC = newPC
                  RB = (state.RB + offset) }
    | Halt -> Complete state

let createState (line: string) =
    let state =
        line.Split(',')
        |> Seq.mapi (fun i s -> (bigint i, bigint.Parse s))
        |> Map.ofSeq
    { PC = bigint.Zero
      RB = bigint.Zero
      State = state }

let hitWall result =
    match result with
    | HasOutput output when output.Output = 0I -> true
    | HasOutput output when output.Output = 1I -> false
    | HasOutput output when output.Output = 2I -> false
    | _ -> failwithf "Checked if hit a wall on %A" result

let isAtOxygenSystem result =
    match result with
    | HasOutput output when output.Output = 0I -> false
    | HasOutput output when output.Output = 1I -> false
    | HasOutput output when output.Output = 2I -> true
    | _ -> failwithf "Checked if at oxygen system on %A" result

let continueFromOutput result =
    match result with
    | HasOutput output -> output.Continue()
    | _ -> failwithf "Tried to continue on %A" result

let getNeighbors (x, y) result =
    match result with
    | AwaitingInput(_, continueFunc) ->
        seq {
            (1I, (x, y + 1))
            (2I, (x, y - 1))
            (3I, (x - 1, y))
            (4I, (x + 1, y))
        }
        |> Seq.choose (fun (i, pos) ->
            let result = continueFunc i
            if hitWall result then None
            else Some(pos, result))
    | _ -> failwithf "Invalid result %A passed to get neighbors at %A" result (x, y)

type Tile =
    | Goal
    | Space

let rec search grid seen frontier =
    match Queue.tryDequeue frontier with
    | None -> grid
    | Some((pos, depth, result), frontier) ->
        if isAtOxygenSystem result then
            Map.add pos (Goal, depth) grid
        else
            let grid = Map.add pos (Space, depth) grid
            let seen = Set.add pos seen

            let frontier =
                result
                |> continueFromOutput
                |> getNeighbors pos
                |> Seq.filter (fun (pos, _) -> Set.contains pos seen |> not)
                |> Seq.map (fun (pos, result) -> (pos, depth + 1, result))
                |> Seq.fold (fun q e -> Queue.enqueue e q) frontier
            search grid seen frontier

let findSolution state =
    search Map.empty Set.empty
        (Queue.ofSeq
            (state
             |> runIntCode
             |> getNeighbors (0, 0)
             |> Seq.map (fun (pos, result) -> (pos, 1, result))
             |> List.ofSeq))
    |> Seq.find (fun entry -> fst entry.Value = Goal)
    |> (fun entry -> snd entry.Value)

[<EntryPoint>]
let main argv =
    argv.[0]
    |> File.ReadLines
    |> Seq.head
    |> createState
    |> findSolution
    |> printfn "%A"
    0
