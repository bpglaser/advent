open System
open System.IO

type Fold =
    | XFold of int
    | YFold of int

let xPrefix = "fold along x="
let yPrefix = "fold along y="

let dims grid =
    Array2D.length1 grid, Array2D.length2 grid

let makeEmptyGrid w h = Array2D.create w h ' '

let buildGrid pairs =
    let maxPair (a, b) (c, d) = (max a c, max b d)
    let maxX, maxY = Array.fold maxPair (0, 0) pairs
    let grid = makeEmptyGrid (maxX + 1) (maxY + 1)

    for (x, y) in pairs do
        grid[x, y] <- '#'

    grid

let parse lines =
    let pair (s: string) =
        match s.Split(',', StringSplitOptions.TrimEntries) with
        | [| x; y |] -> Some(int x, int y)
        | _ -> None

    let fold (s: string) =
        if s.StartsWith xPrefix then
            s.Substring xPrefix.Length |> int |> XFold |> Some
        elif s.StartsWith yPrefix then
            s.Substring yPrefix.Length |> int |> YFold |> Some
        else
            None

    let pairs = Array.choose pair lines
    buildGrid pairs, Array.choose fold lines

let splitHorizontal x grid =
    let w, h = dims grid
    assert (x < w)
    let l = makeEmptyGrid x h
    let r = Array2D.copy l
    Array2D.blit grid 0 0 l 0 0 x h
    Array2D.blit grid (x + 1) 0 r 0 0 (w - x - 1) h
    l, r

let splitVertical y grid =
    let w, h = dims grid
    assert (y < h)
    let u = makeEmptyGrid w y
    let d = Array2D.copy u
    Array2D.blit grid 0 0 u 0 0 w y
    Array2D.blit grid 0 (y + 1) d 0 0 w (h - y - 1)
    u, d

let flipHorizontal grid =
    let w, h = dims grid
    let result = makeEmptyGrid w h

    for y in 0 .. (h - 1) do
        for x in 0 .. (w - 1) do
            result[w - x - 1, y] <- grid[x, y]

    result

let flipVertical grid =
    let w, h = dims grid
    let result = makeEmptyGrid w h

    for y in 0 .. (h - 1) do
        for x in 0 .. (w - 1) do
            result[x, h - y - 1] <- grid[x, y]

    result

let overlay a b : 'b =
    let w, h = dims a
    assert ((w, h) = dims b)
    let result = makeEmptyGrid w h

    for x in 0 .. (w - 1) do
        for y in 0 .. (h - 1) do
            result[x, y] <- if a[x, y] = '#' || b[x, y] = '#' then
                                '#'
                            else
                                ' '

    result

let applyFold grid fold =
    match fold with
    | XFold i ->
        let u, d = splitHorizontal i grid
        let d = flipHorizontal d
        overlay u d
    | YFold i ->
        let l, r = splitVertical i grid
        let r = flipVertical r
        overlay l r

let output grid =
    let w, h = dims grid

    for y in 0 .. h - 1 do
        for x in 0 .. w - 1 do
            printf "%c" grid[x, y]

        printf "\n"

let solve (grid, folds) =
    Array.fold applyFold grid folds

[<EntryPoint>]
let main args =
    args[0]
    |> File.ReadAllLines
    |> parse
    |> solve
    |> output

    0
