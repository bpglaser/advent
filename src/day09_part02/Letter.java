package day09_part02;

public class Letter extends Token {

    public long finalSize() {
        return getMultiplier();
    }

    @Override
    public void multiply(long n) {
        setMultiplier(n);
    }

    @Override
    public int lexicalSize() {
        return 1;
    }
}
