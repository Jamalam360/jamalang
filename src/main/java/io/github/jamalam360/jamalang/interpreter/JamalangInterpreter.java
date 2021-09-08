package io.github.jamalam360.jamalang.interpreter;

import io.github.jamalam360.jamalang.langauge.BuiltInFunctions;

import java.util.HashMap;

@SuppressWarnings("OptionalGetWithoutIsPresent")
// We check if it is present before (using a different method other than isPresent)
public class JamalangInterpreter {
    public final HashMap<String, Double> variables = new HashMap<>();
    private final BuiltInFunctions functions;

    public JamalangInterpreter() {
        this.functions = new BuiltInFunctions(this);
    }

    public void execute(String inputCode) throws Exception {
        String[] lines = inputCode.split("\\n"); // Split line by line
        int lineNumber = 0;

        for (String line : lines) {
            lineNumber++;

            line = ParsingHelper.sanitizeInput(line);
            line = ParsingHelper.makeLineSane(line);

            if (ParsingHelper.shouldExecute(line)) {
                String[] keywords = line.split(" ");

                if (keywords[0].equals("var")) { // Operation is assigning a variable
                    String name = keywords[1];
                    String value = ParsingHelper.makeLineSane(line.split("=")[1]);

                    if (variables.containsKey(name)) {
                        throw InterpreterExceptions.nameAlreadyInUse(name);
                    } else {
                        variables.put(name, evaluateToDouble(value));
                    }
                } else if (ParsingHelper.getFunctionName(keywords[0]).equals("for")) {
                    String[] forArgs = ParsingHelper.getArgArray(ParsingHelper.getArgsInFunction(line.split("=>")[0]));
                    String forVarName = ParsingHelper.getArgArray(ParsingHelper.getArgsInFunction(line.split("=>")[1]))[0];
                    String[] executionCode = ParsingHelper.getEnclosedLines(lines, lineNumber);

                    for (int i = 0; i < Integer.parseInt(forArgs[0]); i++) {
                        for (String forLine : executionCode) {
                            this.execute(forLine);
                        }

                        this.variables.put(forVarName, (double) i);
                    }

                    this.variables.remove(forVarName);
                } else if (variables.containsKey(keywords[0])) { // An assignment operation to a pre-existing variable
                    String name = keywords[0];
                    String operation = keywords[1];
                    String value = "";

                    if (keywords.length > 2) {
                        value = keywords[2];
                    }

                    double doubleValue = variables.get(name);

                    switch (operation) {
                        case "=" -> variables.put(name, evaluateToDouble(value));
                        case "+=" -> variables.put(name, doubleValue + evaluateToDouble(value));
                        case "-=" -> variables.put(name, doubleValue - evaluateToDouble(value));
                        case "++" -> variables.put(name, doubleValue + 1);
                        case "--" -> variables.put(name, doubleValue - 1);
                    }
                } else if (functions.hasFunction(line)) {
                    functions.getFunction(line).get().execute(ParsingHelper.getArgArray(ParsingHelper.getArgsInFunction(line)));
                }
            }
        }
    }

    /**
     * @param value The input to evaluate
     * @return The double representation of the input
     */
    private double evaluateToDouble(String value) throws Exception {
        if (ParsingHelper.isBoolean(value)) {
            return ParsingHelper.evaluateBooleanToDouble(value);
        } else if (functions.hasFunction(value)) {
            return (double) functions.getFunction(value).get().execute(ParsingHelper.getArgArray(ParsingHelper.getArgsInFunction(value)));
        } else {
            if (value.startsWith("!") && variables.containsKey(value.split("!")[1])) {
                return ParsingHelper.invertBoolean(variables.get(value.split("!")[1]));
            } else if (variables.containsKey(value)) {
                return variables.get(value);
            } else {
                return Double.parseDouble(value);
            }
        }
    }
}
