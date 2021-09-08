package io.github.jamalam360.jamalang;

import io.github.jamalam360.jamalang.interpreter.JamalangInterpreter;

import java.io.File;
import java.nio.file.Files;

public class Main {
    private static final File JAMALANG_TEST_FILE = new File("script.jlang");

    public static void main(String[] args) throws Exception {
        new JamalangInterpreter().execute(Files.readString(JAMALANG_TEST_FILE.toPath()));
    }
}
