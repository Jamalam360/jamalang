package io.github.jamalam360.jamalang.langauge;

import io.github.jamalam360.jamalang.interpreter.Exceptions;
import io.github.jamalam360.jamalang.interpreter.JamalangInterpreter;
import io.github.jamalam360.jamalang.interpreter.ParsingHelper;
import io.github.jamalam360.jamalang.util.JamalangFunction;
import io.github.jamalam360.jamalang.util.UserInput;

import java.util.HashMap;
import java.util.Optional;

public class BuiltInFunctions {
    private final JamalangInterpreter interpreter;
    private final HashMap<String, JamalangFunction<?>> builtInFunctions = new HashMap<>();

    public BuiltInFunctions(JamalangInterpreter interpreter) {
        this.interpreter = interpreter;

        builtInFunctions.put("print", (args -> {
            if (args.length != 1) {
                throw Exceptions.incorrectParameterException("print");
            } else {
                if (interpreter.variables.containsKey(args[0])) {
                    String output = String.valueOf(interpreter.variables.get(args[0]));

                    if (output.contains(".0")) {
                        output = output.split("\\.")[0];
                    }

                    System.out.println(output);
                }

                return 1;
            }
        }));

        builtInFunctions.put("sqrt", (args -> {
            if (args.length != 1) {
                throw Exceptions.incorrectParameterException("Sqrt");
            } else {
                if (interpreter.variables.containsKey(args[0])) {
                    double value = interpreter.variables.get(args[0]);

                    return Math.sqrt(value);
                } else {
                    return Math.sqrt(Double.parseDouble(args[0]));
                }
            }
        }));

        builtInFunctions.put("userInput", (args -> {
            if (!args[0].isEmpty()) {
                throw Exceptions.incorrectParameterException("userInput");
            } else {
                return Double.parseDouble(UserInput.getUserInput());
            }
        }));

        builtInFunctions.put("add", (args -> {
            if (interpreter.variables.containsKey(args[0])) {
                double value1 = interpreter.variables.get(args[0]);
                double value2 = interpreter.variables.get(args[1]);

                return value1 + value2;
            } else {
                return Integer.parseInt(args[0]) + Integer.parseInt(args[1]);
            }
        }));
    }

    /**
     * @param input The function call to evaluate e.g. print(aString)
     * @return An optional containing the function, if it is present
     */
    public Optional<JamalangFunction<?>> getFunction(String input) {
        if (builtInFunctions.containsKey(ParsingHelper.getFunctionName(input))) {
            return Optional.of(builtInFunctions.get(ParsingHelper.getFunctionName(input)));
        } else {
            return Optional.empty();
        }
    }

    /**
     * @param input The function call to evaluate e.g. print(aString)
     * @return Whether the function is part of the built-in function set
     */
    public boolean hasFunction(String input) {
        return builtInFunctions.containsKey(ParsingHelper.getFunctionName(input));
    }
}
