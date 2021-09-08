package io.github.jamalam360.jamalang.util;

import io.github.jamalam360.jamalang.interpreter.InterpreterExceptions;

import java.util.Scanner;

public class UserInput {
    private static final Scanner SCANNER = new Scanner(System.in);

    public static String getUserInput() throws Exception {
        if (SCANNER.hasNext()) {
            return SCANNER.nextLine();
        } else {
            throw InterpreterExceptions.userInputFailure();
        }
    }
}
