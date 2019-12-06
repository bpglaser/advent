module Tests

open Expecto
open Program

[<Tests>]
let tests =
    let basicMap = buildMap Map.empty [
        ("COM", "FOO") // 1
        ("COM", "BAR") // 1
        ("FOO", "BIN") // 2
        ("BIN", "BAZ") // 3
    ]

    testList "default" [
        testCase "buildMapBasic" <| fun _ ->
            let actual = basicMap
            let expected = Map.ofList [
                ("COM", ["BAR"; "FOO"])
                ("FOO", ["BIN"])
                ("BIN", ["BAZ"])
            ]
            Expect.equal actual expected "The built map is equal"
        
        testCase "sumOrbitsBasic" <| fun _ ->
            Expect.equal (sumOrbits basicMap 0 "COM") 7 "Orbits are summed correctly"

        testCase "given" <| fun _ ->
            let input = seq {
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
            }
            Expect.equal (solve input) 42 "Given input should solve correctly"
    ]
