open Expecto
open Program

[<Tests>]
let tests = testList "intermediaryPoints" [
    let testCases = [
        {|
            Name = "vert"
            Points = (1, 1, 1, 3)
            Expected = [
                (1, 1)
                (1, 2)
                (1, 3)
            ]
        |}
        {|
            Name = "horiz"
            Points = (9, 7, 7, 7)
            Expected = [
                (9, 7)
                (8, 7)
                (7, 7)
            ]
        |}
        {|
            Name = "rising"
            Points = (1, 1, 3, 3)
            Expected = [
                (1, 1)
                (2, 2)
                (3, 3)
            ]
        |}
        {|
            Name = "backwards"
            Points = (9, 7, 7, 9)
            Expected = [
                (9, 7)
                (8, 8)
                (7, 9)
            ]
        |}
    ]
    for tc in testCases -> test tc.Name {
        let (x1, y1, x2, y2) = tc.Points
        let p1 = { X = x1; Y = y1 }
        let p2 = { X = x2; Y = y2 }
        let actual =
            { P1 = p1; P2 = p2 }
            |> intermediaryPoints
            |> List.ofSeq
        let expected = [
            for (x, y) in tc.Expected ->
                { X = x; Y = y}
        ]
        Expect.equal actual expected "points equal"
    }
]

[<EntryPoint>]
let main argv =
    runTestsInAssemblyWithCLIArgs [] argv
