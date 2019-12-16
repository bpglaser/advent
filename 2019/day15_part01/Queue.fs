module Queue

type 'a Queue = Queue of 'a list * 'a list

let empty = Queue([], [])

let isEmpty =
    function
    | Queue([], []) -> true
    | _ -> false

let enqueue a = function
    | Queue(front, back) -> Queue(front, a :: back)

let dequeue =
    function
    | Queue([], []) -> failwith "Empty queue"
    | Queue(x :: front, tail) -> (x, Queue(front, tail))
    | Queue([], tail) ->
        let front = List.rev tail
        (List.head front, Queue(List.tail front, []))

let tryDequeue =
    function
    | Queue([], []) -> None
    | q -> Some(dequeue q)

let ofSeq s = Seq.fold (fun q e -> enqueue e q) empty s
