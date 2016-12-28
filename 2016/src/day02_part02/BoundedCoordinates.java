package day02_part02;

public class BoundedCoordinates {

    private final char[][] possibilities;
    private int x;
    private int y;

    public BoundedCoordinates(char[][] possibilities, int startX, int startY) {
        this.possibilities = possibilities;
        this.x = startX;
        this.y = startY;
    }

    public int getX() {
        return x;
    }

    private void setX(int newX) {
        if (isWithinBounds(newX, y)) {
            this.x = newX;
        }
    }

    public int getY() {
        return y;
    }

    private void setY(int newY) {
        if (isWithinBounds(x, newY)) {
            this.y = newY;
        }
    }

    private boolean isWithinBounds(int newX, int newY) {
        try {
            return possibilities[newY][newX] != ' ';
        } catch (IndexOutOfBoundsException ignored) {
            return false;
        }
    }

    public void move(Direction direction) {
        int newX = x;
        int newY = y;
        switch (direction) {
            case UP:
                newY = y - 1;
                break;
            case RIGHT:
                newX = x + 1;
                break;
            case DOWN:
                newY = y + 1;
                break;
            case LEFT:
                newX = x - 1;
                break;
        }
        setX(newX);
        setY(newY);
    }
}
