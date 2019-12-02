open System.IO

let applyInitialChanges (state: int array) =
    state.[1] <- 12
    state.[2] <- 2
    state

let rec runIntCode (state: int array) pc =
    let apply func =
        state.[state.[pc + 3]] <- func state.[state.[pc + 1]] state.[state.[pc + 2]]

    match state.[pc] with
    | 1 ->
        apply (+)
        runIntCode state (pc + 4)
    | 2 ->
        apply (*)
        runIntCode state (pc + 4)
    | 99 -> state.[0]
    | _ -> failwithf "Invalid opcode encountered state[%i]=%i" pc state.[pc]

[<EntryPoint>]
let main argv =
    argv.[0]
    |> File.ReadLines
    |> Seq.head
    |> (fun s -> s.Split(','))
    |> Array.map int
    |> applyInitialChanges
    |> (fun state -> runIntCode state 0)
    |> printfn "%i"
    0
