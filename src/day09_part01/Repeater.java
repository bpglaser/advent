package day09_part01;

public class Repeater {

    private final int startIndex;
    private final int endIndex;

    private int updatedIndex;

    public Repeater(int startIndex, int endIndex) {
        this.startIndex = startIndex;
        this.endIndex = endIndex;
    }

    public String parseRepeats(String inputString) {
        String instruction = inputString.substring(startIndex + 1, endIndex);

        int splitIndex = instruction.indexOf('x');
        int charCount = Integer.parseInt(instruction.substring(0, splitIndex));
        int repeatCount = Integer.parseInt(instruction.substring(splitIndex + 1));

        String repeatedString = inputString.substring(endIndex + 1, endIndex + 1 + charCount);

        StringBuilder sb = new StringBuilder();
        for (int j = 0; j < repeatCount; j++) {
            sb.append(repeatedString);
        }

        updatedIndex = endIndex + charCount;

        return sb.toString();
    }

    public int getUpdatedIndex() {
        return updatedIndex;
    }
}
