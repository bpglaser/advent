package day02_part01;

import java.util.List;

public class KeypadSolver {

    public static final char[][] KEYPAD = {
            {'1', '2', '3'},
            {'4', '5', '6'},
            {'7', '8', '9'},
    };

    private final List<List<Direction>> directions;
    private final BoundedCoordinates coordinates;

    public KeypadSolver(List<List<Direction>> directions) {
        this.directions = directions;
        this.coordinates = new BoundedCoordinates(1, 1, 0, 2);
    }

    public String solve() {
        StringBuilder stringBuilder = new StringBuilder();
        for (List<Direction> line : directions) {
            stringBuilder.append(solveLine(line));
        }
        return stringBuilder.toString();
    }

    private char solveLine(List<Direction> line) {
        for (Direction direction : line) {
            coordinates.move(direction);
        }
        return KEYPAD[coordinates.getY()][coordinates.getX()];
    }
}
