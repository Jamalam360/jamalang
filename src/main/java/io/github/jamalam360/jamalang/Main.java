package io.github.jamalam360.jamalang;

import io.github.jamalam360.jamalang.interpreter.JamalangInterpreter;

import java.io.File;
import java.nio.file.Files;

public class Main {
    public static void main(String[] args) throws Exception {
        new JamalangInterpreter().execute(Files.readString(new File(args[0]).toPath()));
    }

    //region To-Do:

    //TODO: Clean up and split up ParsingHelper into its own categories
    //TODO: Abstract variable management into its own class
    //TODO: String support
    //TODO: Mathematical Expression Evaluator
    //TODO: Nested Loops
    //TODO: While loops
    //TODO: Validation of parameters
    //TODO: Classes
    //TODO: Custom functions

    //endregion
}
