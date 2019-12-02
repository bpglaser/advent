open System.IO

let rec runIntCode pc (state: int array) =
    let apply func =
        state.[state.[pc + 3]] <- func state.[state.[pc + 1]] state.[state.[pc + 2]]

    match state.[pc] with
    | 1 ->
        apply (+)
        runIntCode (pc + 4) state
    | 2 ->
        apply (*)
        runIntCode (pc + 4) state
    | 99 -> state.[0]
    | _ -> failwithf "Invalid opcode encountered state[%i]=%i" pc state.[pc]

let applyInitialChanges (initialState: int array) (i, j) =
    let clone = Array.copy initialState
    clone.[1] <- i
    clone.[2] <- j
    clone

let findSolution initialState =
    let inputs = [| for i in 0..99 do for j in 0..99 -> (i, j) |]
    let idx =
        inputs
        |> Array.Parallel.map
            (applyInitialChanges initialState >> runIntCode 0)
        |> Array.findIndex ((=) 19690720)
    let i, j = inputs.[idx]
    i * 100 + j

[<EntryPoint>]
let main argv =
    argv.[0]
    |> File.ReadLines
    |> Seq.head
    |> (fun s -> s.Split(','))
    |> Array.map int
    |> findSolution
    |> printfn "%i"
    0
