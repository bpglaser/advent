package day09_part01;

import java.io.BufferedReader;
import java.io.FileInputStream;
import java.io.IOException;
import java.io.InputStreamReader;

public class Day09Part01 {
    public static void main(String[] args) throws IOException {
        String input = readInput(args[0]);

        RepeaterBuilder builder = null;
        StringBuilder sb = new StringBuilder();

        for (int i = 0; i < input.length(); i++) {
            char c = input.charAt(i);
            if (c == '(' && builder == null) {
                builder = new RepeaterBuilder(i);
                continue;
            }
            if (builder != null) {
                if (c == ')') {
                    builder.setEndIndex(i);
                    Repeater repeater = builder.build();
                    builder = null;

                    sb.append(repeater.parseRepeats(input));
                    i = repeater.getUpdatedIndex();
                }
                continue;
            }

            sb.append(c);
        }

        String result = sb.toString();
        System.out.println("result: " + result);
        System.out.println("length: " + result.length());
    }

    public static String readInput(String path) throws IOException {
        BufferedReader bufferedReader = new BufferedReader(new InputStreamReader(new FileInputStream(path)));
        return bufferedReader.readLine();
    }
}
