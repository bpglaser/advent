package day09_part02;

public class RepeaterBuilder {

    private final int startIndex;

    private int endIndex;

    public RepeaterBuilder(int startIndex) {
        this.startIndex = startIndex;
    }

    public void setEndIndex(int endIndex) {
        this.endIndex = endIndex;
    }

    public Repeater build(String input) {
        String instruction = input.substring(startIndex + 1, endIndex);
        int splitIndex = instruction.indexOf('x');
        int subsequentLexicalCount = Integer.parseInt(instruction.substring(0, splitIndex));
        int repeatCount = Integer.parseInt(instruction.substring(splitIndex + 1));
        return new Repeater(subsequentLexicalCount, repeatCount, endIndex - startIndex + 1);
    }
}
