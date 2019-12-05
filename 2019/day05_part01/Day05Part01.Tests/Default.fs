module Tests

open Expecto
open Program

[<Tests>]
let tests =
    testList "default"
        [ testCase "basic add parse" <| fun _ ->
            let input = [| 1; 0; 0; 0 |]
            let actual = parseInstruction 0 input

            let expected =
                { Op = Add
                  Params =
                      [| (0, Position)
                         (0, Position)
                         (0, Position) |] }
            Expect.equal actual expected "Add should be parsed"

          testCase "advanced parsing" <| fun _ ->
              let ops =
                  [ 1, Add
                    2, Mul ]

              let modes =
                  [ 000, Position, Position, Position
                    001, Position, Position, Immediate
                    010, Position, Immediate, Position
                    011, Position, Immediate, Immediate
                    100, Immediate, Position, Position
                    101, Immediate, Position, Immediate
                    110, Immediate, Immediate, Position
                    111, Immediate, Immediate, Immediate ]

              let results =
                  List.allPairs ops modes
                  |> List.map (fun ((i, op), (j, a, b, c)) ->
                      let actual =
                          parseInstruction 0
                              [| i + j * 100
                                 0
                                 0
                                 0 |]

                      let expected =
                          { Op = op
                            Params =
                                [| 0, a
                                   0, b
                                   0, c |] }
                      actual = expected)

              Expect.all results id "All modes parsed"

          testCase "given1" <| fun _ ->
              let actual = runIntCode 0 [| 1; 9; 10; 3; 2; 3; 11; 0; 99; 30; 40; 50 |]
              Expect.equal actual 3500 "Given test case passes"

          testCase "given2" <| fun _ ->
              let actual = runIntCode 0 [| 1; 0; 0; 0; 99 |]
              Expect.equal actual 2 "Given test case passes"

          testCase "given3" <| fun _ ->
              let actual = runIntCode 0 [| 1; 1; 1; 4; 99; 5; 6; 0; 99 |]
              Expect.equal actual 30 "Given test case passes" ]
