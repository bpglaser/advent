package day02_part01;

public class BoundedCoordinates {

    private final int min;
    private final int max;
    private int x;
    private int y;

    public BoundedCoordinates(int initialX, int initialY, int min, int max) {
        this.x = initialX;
        this.y = initialY;
        this.min = min;
        this.max = max;
    }

    public int getX() {
        return x;
    }

    public int getY() {
        return y;
    }

    public void move(Direction direction) {
        switch (direction) {
            case UP:
                if ((y - 1) >= min) {
                    y -= 1;
                }
                break;
            case RIGHT:
                if ((x + 1) <= max) {
                    x += 1;
                }
                break;
            case DOWN:
                if ((y + 1) <= max) {
                    y += 1;
                }
                break;
            case LEFT:
                if ((x - 1) >= min) {
                    x -= 1;
                }
                break;
        }
    }
}
