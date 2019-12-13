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
    | HasOutput output -> (output.Output, output.Continue)
    | _ -> failwithf "Tried to get output from %A" result

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
    if tileID = 0I then Some Empty
    elif tileID = 1I then Some Wall
    elif tileID = 2I then Some Block
    elif tileID = 3I then Some Paddle
    elif tileID = 4I then Some Ball
    else None

let tileChar tile =
    match tile with
    | Empty -> ' '
    | Wall -> '#'
    | Block -> 'B'
    | Paddle -> '-'
    | Ball -> 'O'

let createState (line: string) =
    let state =
        line.Split(',')
        |> Seq.mapi (fun i s -> (bigint i, bigint.Parse s))
        |> Map.ofSeq
    { PC = 0I
      RB = 0I
      State = state }

let scoreKey = (-1I, 0I)

let render grid =
    let score = Map.tryFind scoreKey grid |> Option.defaultValue 0I
    let grid = Map.remove scoreKey grid
    let minX = grid |> Seq.map (fun entry -> fst entry.Key) |> Seq.min
    let maxX = grid |> Seq.map (fun entry -> fst entry.Key) |> Seq.max
    let minY = grid |> Seq.map (fun entry -> snd entry.Key) |> Seq.min
    let maxY = grid |> Seq.map (fun entry -> snd entry.Key) |> Seq.max

    printfn "Score: %A" score

    for y in minY .. maxY do
        for x in minX .. maxX do
            grid |> Map.find (x, y) |> tile |> Option.get |> tileChar |> printf "%c"
        printfn ""

let step state = 
    let rec inner grid result =
        match result with
        | AwaitingInput cont ->
            let (ballX, _) = Map.findKey (fun _ value -> tile value = Some Ball) grid
            let (paddleX, _) = Map.findKey (fun _ value -> tile value = Some Paddle) grid
            let input =
                if ballX < paddleX then -1I
                elif ballX = paddleX then 0I
                else 1I
            inner grid (cont input)
        | Complete ->
            Map.tryFind scoreKey grid |> Option.defaultValue 0I
        | HasOutput output ->
            let x = output.Output
            let (y, cont) = output.Continue() |> getOutput
            let (value, cont) = cont() |> getOutput
            inner (Map.add (x, y) value grid) (cont())

    state |> runIntCode |> inner Map.empty

let findSolution state =
    let state = { state with State = Map.add 0I 2I state.State }
    step state

[<EntryPoint>]
let main argv =
    argv.[0]
    |> File.ReadLines
    |> Seq.head
    |> createState
    |> findSolution
    |> printfn "%A"
    0
