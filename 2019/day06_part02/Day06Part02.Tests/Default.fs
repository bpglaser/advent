module Tests

open Expecto
open Program

[<Tests>]
let tests =
    let given =
        seq {
            "COM)B"
            "B)C"
            "C)D"
            "D)E"
            "E)F"
            "B)G"
            "G)H"
            "D)I"
            "E)J"
            "J)K"
            "K)L"
            "K)YOU"
            "I)SAN" } |> parseLines

    testList "default" [
        testCase "basicFindPath" <| fun _ ->
            let actual = findPath given "COM" "B"
            let expected = Some ["COM"]
            Expect.equal actual expected "List was found"

        testCase "advancedFindPath1" <| fun _ ->
            let actual = findPath given "COM" "YOU"
            let expected = Some ["COM"; "B"; "C"; "D"; "E"; "J"; "K"]
            Expect.equal actual expected "List was found"

        testCase "advancedFindPath2" <| fun _ ->
            let actual = findPath given "COM" "SAN"
            let expected = Some ["COM"; "B"; "C"; "D"; "I"]
            Expect.equal actual expected "List was found"
        
        testCase "countTransfers" <| fun _ ->
            let actual = countTransfers given "YOU" "SAN"
            Expect.equal actual (Some 4) "Found the correct number of transfers"
    ]
