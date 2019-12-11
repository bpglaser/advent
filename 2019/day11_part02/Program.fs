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

let getOutput result =
    match result with
    | HasOutput output -> output
    | _ -> failwith "Tried to get output from incorrect result."

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
    // printfn "%A" state.PC
    // printfn "%A" [ for i in 0 .. 3 -> state.[state.PC + bigint i] ]
    // printfn "%A" instruction
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
        AwaitingInput continueExec
    | Output ->
        let continueExec() = runIntCode { state with PC = newPC }
        HasOutput
            { Output = (get instruction.Params.[0])
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
    | Halt -> Complete

let createState (line: string) =
    let state =
        line.Split(',')
        |> Seq.mapi (fun i s -> (bigint i, bigint.Parse s))
        |> Map.ofSeq
    { PC = bigint.Zero
      RB = bigint.Zero
      State = state }

type Facing =
    | Up
    | Down
    | Left
    | Right

let move (x, y) facing output =
    if output = bigint.Zero then  // turn left
        match facing with
        | Up -> ((x - 1, y), Left)
        | Down -> ((x + 1, y), Right)
        | Left -> ((x, y - 1), Down)
        | Right -> ((x, y + 1), Up)
    elif output = bigint.One then  // turn right
        match facing with
        | Up -> ((x + 1, y), Right)
        | Down -> ((x - 1, y), Left)
        | Left -> ((x, y + 1), Up)
        | Right -> ((x, y - 1), Down)
    else
        failwithf "Invalid output movement: %A" output

let getGridDisplayString (grid: Map<int * int, bigint * int>) =
    let displayChars =
        Map.ofList
            [ (bigint.Zero, ' ')
              (bigint.One, '#') ]

    let activePositions =
        grid
        |> Seq.filter (fun entry -> (fst entry.Value) = bigint.One)
        |> Seq.map (fun entry -> entry.Key)
        |> List.ofSeq

    let top =
        activePositions
        |> List.map snd
        |> List.max

    let bottom =
        activePositions
        |> List.map snd
        |> List.min

    let left =
        activePositions
        |> List.map fst
        |> List.min

    let right =
        activePositions
        |> List.map fst
        |> List.max

    printfn "%A" (top, bottom, left, right)

    let mutable rows = []
    for y in top + 1 .. -1 .. bottom - 1 do
        let mutable row = []
        for x in left - 1 .. right + 1 do
            row <-
                (Map.tryFind (x, y) grid
                 |> Option.map fst
                 |> Option.defaultValue bigint.Zero)
                :: row
        rows <-
            (row
             |> List.rev
             |> List.map (fun i -> Map.find i displayChars)
             |> String.Concat)
            :: rows
    String.Join('\n', List.rev rows)

let findSolution state =
    let rec inner result grid position facing =
        match result with
        | AwaitingInput func ->
            let (color, _) = Map.tryFind position grid |> Option.defaultValue (bigint.Zero, 0)
            inner (func color) grid position facing
        | Complete -> grid |> getGridDisplayString
        | HasOutput output ->
            // get the number of times the current square has been painted
            let paintCount =
                Map.tryFind position grid
                |> Option.map snd
                |> Option.defaultValue 0
            // paint the current square and increase the paint count
            let grid = Map.add position (output.Output, paintCount + 1) grid
            // get the next output (the rotate command)
            let output = output.Continue() |> getOutput
            // do the move
            let (position, facing) = move position facing output.Output
            // run from the new state
            inner (output.Continue()) grid position facing

    inner (runIntCode state) (Map.ofList [ ((0, 0), (bigint.One, 0)) ]) (0, 0) Up

[<EntryPoint>]
let main argv =
    argv.[0]
    |> File.ReadLines
    |> Seq.head
    |> createState
    |> findSolution
    |> printfn "%s"
    0
