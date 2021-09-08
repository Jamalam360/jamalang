package io.github.jamalam360.jamalang.interpreter;

public class InterpreterExceptions {
    public static Exception nameAlreadyInUse(String name) {
        return new UnsupportedOperationException("Cannot create a new variable with the name %s because that name is already in use".formatted(name));
    }

    public static Exception userInputFailure() {
        return new RuntimeException("Error reading user input");
    }

    public static Exception incorrectParameterException(String methodName) {
        return new UnsupportedOperationException("Incorrect parameters for %s".formatted(methodName));
    }
}
