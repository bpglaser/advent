module Tests

open Expecto
open Program

[<Tests>]
let tests =
    testList "default" [
        testCase "apply gravity" <| fun _ ->
            let actual = applyGravity [
                { Position = [| 2; -8;  0|]; Velocity = [|-3; -2;  1|] }
                { Position = [| 2;  1;  7|]; Velocity = [| 2;  1;  1|] }
            ]
            let expected = [
                { Position = [| 2; -8;  0|]; Velocity = [|-3; -1;  2|] }
                { Position = [| 2;  1;  7|]; Velocity = [| 2;  0;  0|] }
            ]
            Expect.equal actual expected "Gravity was correctly calculated"

        testCase "update velocity" <| fun _ ->
            let actual = updateVelocity [{ Position = [| 2; -8;  0|]; Velocity = [|-3; -2;  1|] }]
            let expected = [{ Position = [| -1; -10;  1|]; Velocity = [|-3; -2;  1|] }]
            Expect.equal actual expected "Velocity updated correctly"
            
        testCase "single step" <| fun _ ->
            // pos=<x= 2, y=-8, z= 0>, vel=<x=-3, y=-2, z= 1>
            // pos=<x= 2, y= 1, z= 7>, vel=<x= 2, y= 1, z= 1>
            // pos=<x= 2, y= 3, z=-6>, vel=<x= 0, y= 2, z=-1>
            // pos=<x= 2, y=-9, z= 1>, vel=<x= 1, y=-1, z=-1>
            let moons = [
                { Position = [| 2; -8;  0|]; Velocity = [|-3; -2;  1|] }
                { Position = [| 2;  1;  7|]; Velocity = [| 2;  1;  1|] }
                { Position = [| 2;  3; -6|]; Velocity = [| 0;  2; -1|] }
                { Position = [| 2; -9;  1|]; Velocity = [| 1; -1; -1|] }
            ]
            let actual = step moons
            // pos=<x=-1, y=-9, z= 2>, vel=<x=-3, y=-1, z= 2>
            // pos=<x= 4, y= 1, z= 5>, vel=<x= 2, y= 0, z=-2>
            // pos=<x= 2, y= 2, z=-4>, vel=<x= 0, y=-1, z= 2>
            // pos=<x= 3, y=-7, z=-1>, vel=<x= 1, y= 2, z=-2>
            let expected = [
                { Position = [|-1; -9;  2|]; Velocity = [|-3; -1;  2|] }
                { Position = [| 4;  1;  5|]; Velocity = [| 2;  0; -2|] }
                { Position = [| 2;  2; -4|]; Velocity = [| 0; -1;  2|] }
                { Position = [| 3; -7; -1|]; Velocity = [| 1;  2; -2|] }
            ]
            Expect.equal actual expected "A single step should work"
        
        testCase "given1" <| fun _ ->
            let input = "<x=-1, y=0, z=2>\n<x=2, y=-10, z=-7>\n<x=4, y=-8, z=8>\n<x=3, y=5, z=-1>"
            let actual = input.Split('\n') |> parseInput |> findSolution 10
            let expected = 179
            Expect.equal actual expected "Given example should work"

        testCase "given2" <| fun _ ->
            let input = "<x=-8, y=-10, z=0>\n<x=5, y=5, z=10>\n<x=2, y=-7, z=3>\n<x=9, y=-8, z=-3>"
            let actual = input.Split('\n') |> parseInput |> findSolution 100
            let expected = 1940
            Expect.equal actual expected "Given example should work"
    ]
