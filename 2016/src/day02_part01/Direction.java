package day02_part01;

enum Direction {
    UP,
    RIGHT,
    DOWN,
    LEFT;

    public static Direction parse(char c) {
        switch (c) {
            case 'U':
                return UP;
            case 'R':
                return RIGHT;
            case 'D':
                return DOWN;
            case 'L':
                return LEFT;
            default:
                throw new IllegalArgumentException("Tried to parse invalid letter");
        }
    }
}
