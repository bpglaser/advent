package day09_part02;

public abstract class Token {

    private long multiplier = 1;

    public void multiply(long n) {
        multiplier *= n;
    }

    public long getMultiplier() {
        return multiplier;
    }

    public void setMultiplier(Long n) {
        multiplier = n;
    }

    public abstract int lexicalSize();
}
