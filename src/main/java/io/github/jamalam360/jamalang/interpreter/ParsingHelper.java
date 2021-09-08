package io.github.jamalam360.jamalang.interpreter;

import java.util.ArrayList;

public class ParsingHelper {
    /**
     * @param input The string to sanitize
     * @return A cleaned up version of input with end comments removed etc.
     */
    public static String sanitizeInput(String input) {
        if (input.contains("#")) {
            String returnString = input.split("#")[0];
            returnString = returnString.trim();

            return returnString;
        } else {
            return input;
        }
    }

    /**
     * @param input The line to evaluate
     * @return Whether the interpreter should execute that line
     */
    public static boolean shouldExecute(String input) {
        return !input.startsWith("#");
    }

    /**
     * @param input The function call to evaluate e.g. print(aString)
     * @return The name of the function e.g. print
     */
    public static String getFunctionName(String input) {
        return input.trim().split("\\(", 2)[0];
    }

    private static String getArgsInFunction(String input) {
        input = input.split("\\(", 2)[1];
        input = input.split("\\)", 2)[0];
        return input;
    }

    /**
     * @param input The function call to evaluate e.g. print(aString)
     * @return The parameters passed to the function e.g. [aString]
     */
    public static String[] getArgArray(String input) {
        String[] arr = getArgsInFunction(input).split(",");

        for (int i = 0; i < arr.length; i++) {
            arr[i] = sanitizeInput(arr[i]);
            arr[i] = arr[i].trim();
        }

        return arr;
    }

    public static String[] getEnclosedLines(String[] lines, int lineNumber) {
        String nextLine = "";
        ArrayList<String> enclosedLines = new ArrayList<>();

        while (!nextLine.equals("}")) {
            nextLine = lines[lineNumber];
            enclosedLines.add(nextLine);
            lineNumber++;
        }

        return enclosedLines.toArray(new String[0]);
    }

    /**
     * @param value The boolean value to convert to a double
     * @return The double representation of the boolean
     */
    public static double evaluateBooleanToDouble(String value) {
        if (value.startsWith("!")) {
            return value.equals("true") ? 0 : 1;
        } else {
            return value.equals("true") ? 1 : 0;
        }
    }

    public static double invertBoolean(double value) {
        return value == 0 ? 1 : 0;
    }

    /**
     * @param value The value to test
     * @return Whether the value is a valid, plain, boolean
     */
    public static boolean isBoolean(String value) {
        return value.equals("true") || value.equals("false") || value.equals("!true") || value.equals("!false");
    }

    public static String makeLineSane(String value) {
        StringBuilder builder = new StringBuilder();
        boolean lastCharWasSpace = false;

        for (int i = 0; i < value.length(); i++) {
            char nextChar = value.charAt(i);

            if (!(nextChar == ' ' && lastCharWasSpace)) {
                builder.append(nextChar);
                lastCharWasSpace = nextChar == ' ';
            } else {
                lastCharWasSpace = true;
            }
        }

        return builder.toString();
    }
}
