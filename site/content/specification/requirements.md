# Jamalang Implementation Requirements

The following document contains information about the requirements (in terms of
features) for a specification compliant implementation of a Jamalang transpiler.

Jamalang has a number of tests available as well, more information on which can
be found in the [tests](./tests.md) document.

## Basic Syntax

### Comments

Comments are formatted as follows. Jamalang doesn't contain a multi-line comment
syntax.

```
# This is a comment
x = 5 # This is also a comment
```

Comments can be appended at the end of any line after the statement on that line
is completed, or at the start of that line.

### Variables

Variables are declared in the following format:

```
[NAME]: [TYPE] = [EXPRESSION]
```

Where `[TYPE]` is the type of the variable, which is optional, `[NAME]` is the name of the
variable, and `[EXPRESSION]` is the value of the variable.

For example:

```
# UnsignedInt is the default type of a non-negative integer constant.
x = 5
```

Variable names **strictly** use `lower_snake_case`. Variable names **must not**
contain upper case characters.

### Constants

Constants are defined in the same was as variables, but use `UPPER_SNAKE_CASE`.

### Control Flow

If blocks are defined as such:

```
if expression {
   
} elif expression2 {

} else {

}
```

### Loops

Jamalang has both for loops and while loops:

```
for elem in array {

}

while condition {

}
```

### Arrays

Arrays are defined as such:

```
array = [1, 2, 3, 4, 5]

for elem in array {
  # ...
}
```

There is also a range operator. The following code would have the same effect:

```
# LB is inclusive, UB is non-inclusive
array = 1..6
```

### Functions

Functions are defined in the following form:

```
fn example_function(name: String): String {
  # Do things with name
  return name
}
```

If a function doesn't specify a return type, it can be assumed to be Void.

#### Lambda Functions

If a function specifies a function as its last argument, that function can be used as a lambda function:

```
fn example_lambda(glob: String, consumer: (String, String): Void) {
  # Find files matching glob...
  for file in files {
    consumer(file.name, file.extension)
  }
}

example_lambda("*.json") -> (name, ext) {
  # Do things with name and ext
}
```

### Operations

Jamalang contains the following operators. The name of their constraint is also included.

- `+` (Addable)
- `-` (Subtractable)
- `/` (Dividable)
- `*` (Multiplicatable)
- `**` (Powerable)
- `%` (Modulusable)
- `..` (Rangeable)
- `<` (Comparable)
- `>` (Comparable)
- `<=` (Comparable)
- `>=` (Comparable)
- `is` (Comparable)
- `is not` (Comparable)

## Builtins

Jamalang runtime's need to contain a few minimum builtins - the standard library
can't do everything.

### Types (Primitives)

#### `UnsignedInt`

A 32-bit unsigned integer.

#### `SignedInt`

A 32-bit signed integer.

#### `Long`

A 64-bit signed integer.

#### `Float`

A 32-bit floating point number.

#### `Double`

A 64-bit floating point number.

#### `Boolean`

A boolean value.

#### `Char`

A Unicode scalar value, similar to Rust's `char` type.

### Types (Other)

#### `Void`

An empty value. This is used as the type of a function that does not return a
value. Variables **cannot** be assigned this type.

#### `Object`

The base type of any object (i.e. any value _other_ than primitives). This is
used, for example, to define functions that accept any object as an argument.

### Types (Core Library)

The core module of the standard library contains a few types that need to be known by the compiler:

- `String` (this also needs custom syntax)
