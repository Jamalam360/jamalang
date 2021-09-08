package io.github.jamalam360.jamalang;

import io.github.jamalam360.jamalang.interpreter.JamalangInterpreter;

import java.io.File;
import java.nio.file.Files;

public class Main {
    public static void main(String[] args) throws Exception {
        new JamalangInterpreter().execute(Files.readString(new File(args[0]).toPath()));
    }
}
