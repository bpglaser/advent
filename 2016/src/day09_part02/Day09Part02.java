package day09_part02;

import java.io.BufferedReader;
import java.io.FileInputStream;
import java.io.IOException;
import java.io.InputStreamReader;
import java.util.ArrayList;
import java.util.List;

public class Day09Part02 {

    public static void main(String[] args) throws IOException {
        String input = readInput(args[0]);
        List<Token> tokens = new ArrayList<>();

        RepeaterBuilder builder = null;

        for (int i = 0; i < input.length(); i++) {
            char c = input.charAt(i);
            if (c == '(' && builder == null) {
                builder = new RepeaterBuilder(i);
                continue;
            }
            if (builder != null) {
                if (c == ')') {
                    builder.setEndIndex(i);
                    Repeater repeater = builder.build(input);
                    builder = null;
                    tokens.add(repeater);
                }
                continue;
            }
            tokens.add(new Letter());
        }

        for (int tokenIndex = 0; tokenIndex < tokens.size(); tokenIndex++) {
            Token token = tokens.get(tokenIndex);
            if (token instanceof Repeater) {
                Repeater repeater = (Repeater) token;

                int workingIndex = tokenIndex + 1;
                int subsequentLexicalCount = repeater.getSubsequentLexicalCount();
                while (subsequentLexicalCount > 0) {
                    Token tokenToModify = tokens.get(workingIndex);
                    repeater.modify(tokenToModify);
                    subsequentLexicalCount -= tokenToModify.lexicalSize();
                    workingIndex++;
                }
            }
        }

        long decompressedLength = 0;
        for (Token token : tokens) {
            if ((token instanceof Letter)) {
                decompressedLength += ((Letter) token).finalSize();
            }
        }
        System.out.println("decompressed length: " + decompressedLength);
    }

    public static String readInput(String path) throws IOException {
        BufferedReader bufferedReader = new BufferedReader(new InputStreamReader(new FileInputStream(path)));
        return bufferedReader.readLine();
    }
}
