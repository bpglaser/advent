package day09_part01;

public class RepeaterBuilder {

    private int startIndex;
    private int endIndex;

    public RepeaterBuilder(int startIndex) {
        this.startIndex = startIndex;
    }

    public void setEndIndex(int endIndex) {
        this.endIndex = endIndex;
    }

    public Repeater build() {
        return new Repeater(startIndex, endIndex);
    }
}
