package day09_part02;

public class Repeater extends Token {

    private final int subsequentLexicalCount;
    private final int lexicalSize;

    private long runningSum;

    public Repeater(int subsequentLexicalCount, int multiplier, int lexicalSize) {
        super();
        this.subsequentLexicalCount = subsequentLexicalCount;
        multiply(multiplier);
        this.lexicalSize = lexicalSize;

        this.runningSum = multiplier;
    }

    public int getSubsequentLexicalCount() {
        return subsequentLexicalCount;
    }

    public void modify(Token tokenToModify) {
        if (tokenToModify instanceof Repeater) {
            Repeater repeaterToModify = (Repeater) tokenToModify;
            repeaterToModify.runningSum *= getMultiplier();
        } else {
            Letter letterToModify = (Letter) tokenToModify;
            letterToModify.multiply(runningSum);
        }
    }

    @Override
    public int lexicalSize() {
        return lexicalSize;
    }
}
