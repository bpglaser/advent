open System
open System.IO

let solve (width, height) pixels =
    let layers = 
        pixels
        |> Seq.chunkBySize (width * height)
    let result = [| for _ in 1 .. height -> [| for _ in 1 .. width -> 2 |] |]
    for layer in layers do
        for y in 0 .. height - 1 do
            for x in 0 .. width - 1 do
                if result.[y].[x] = 2 then
                    result.[y].[x] <- layer.[y * width + x]
    result

let pixelate =
    function
    | 0 -> ' '
    | 1 -> '*'
    | i -> failwithf "Invalid pixel %i" i

let decodeImage (img: int array array) =
    String.Join('\n', (img |> Seq.map (Seq.map pixelate >> String.Concat)))

[<EntryPoint>]
let main argv =
    argv.[0]
    |> File.ReadLines
    |> Seq.head
    |> Seq.map (string >> Int32.Parse)
    |> solve (25, 6)
    |> decodeImage
    |> printfn "%s"
    0
