package day02_part01;

import java.util.ArrayList;
import java.util.List;
import java.util.Scanner;

public class Day02Part01 {

    public static void main(String[] args) {
        List<List<Direction>> directions = readDirections();
        KeypadSolver keypadSolver = new KeypadSolver(directions);
        String solution = keypadSolver.solve();
        System.out.println(solution);
    }

    private static List<List<Direction>> readDirections() {
        List<List<Direction>> directions = new ArrayList<>();
        Scanner scanner = new Scanner(System.in);
        String line;
        while (!(line = scanner.nextLine()).isEmpty()) {
            ArrayList<Direction> directionLine = new ArrayList<>();
            directions.add(directionLine);
            for (char c : line.toCharArray()) {
                directionLine.add(Direction.parse(c));
            }
        }
        return directions;
    }
}
