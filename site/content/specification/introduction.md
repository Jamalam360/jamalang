# Jamalang - A Toy Language

## Built-in Types

### `Bool`

A one-bit integer - true or false.

### `SignedInt`

The `SignedInt` type is a 32-bit signed integer (i.e. it can be negative).

### `Long`

The `Long` type is a 64-bit signed integer.

### `Float`

The `Float` type is a 32-bit floating point number.

### `Double`

The `Double` type is a 64-bit floating point number.

### `Boolean`

The `Boolean` type is a `true` or `false` value.

### `Char`

The `Char` type is a single unicode character.

### `Void`

The `Void` type is the type of a function that does not return a value, and of
an unassigned value (similar to `null` or `undefined` in other languages).

### `Object`

The `Object` type is the base type of all objects. It can be used for types that
generically accept any object.

## Arrays

Arrays are a collection of values of the same type. They are not dynamic - the
`List` type of the standard library is.

## Type and Constraint System

The constraint system is a way of defining the behavior and abilities of a type
(which can eventually be used to define a new object). To best explain the
constraint system, here is a `Person` type:

```
type Person satisfies StringConvertible, Equatable, Comparable {
    private UnsignedInt id
    UnsignedInt age
    Float height
    Person[] friends

    override String to_string(Person self) {
        return "
            Person {
                id: {self.id},
                age: {self.age},
                height: {self.height},
                friends: {self.friends}
            }
        ".trim()
    }

    override Bool equals(Person self, Person other) {
        return self.id == other.id
    }

    override SignedInt compare(Person self, Person other) {
        return self.id - other.id
    }
}
```

The `Person` type is defined using the `type` keyword, and is defined to satisfy
the constraints `StringConvertible`, `Equatable`, and `Comparable`.

As can be seen by the `override` keyword, `to_string`, `equals`, and `compare`
are all methods that are defined in the `StringConvertible`, `Equatable`, and
`Comparable` constraints, respectively. This means that the `Person` type must
implement these methods.

To extend on the example, since the `Person` type satisfies `Equatable`, we
could use it in an `if` statement:

```
if person1 is person2 {
    // ...
}
```

The `if` statement would call the `equals` method on `person1` and `person2`.

Another example is that, since the `Person` satisfies the `StringConvertible`
constraint, we could use it in a `print` statement. For the sake of this
example, we'll define our own `print` function:

```
Void print(StringConvertable value) {
    //
}

print(person1)
```

### Type Aliases

Type-aliases can be used to give types more descriptive names or shorten
commonly used types:

```
type ID = alias UnsignedInt
type StringConsumer = alias (String): Void
```

## Imports and WASM

Jamalang has the ability to import from other files, such as the standard
library.

[TODO]

## Functions as Parameters

In Jamalang, functions can be defined as the last argument to another function.

For example,

```
fn use_hello(append: String, func: (String): Void) {
    func("Hello " + append)
}

use_hello("World") -> (str) { 
    print(str)
}

# This is a more helpful example:

files_matching("**/*.json") -> (file) {
    file.delete()
}
```

## Examples

```
use Random from "https://raw.githubusercontent.com/Jamalam360/jamalang/main/std/random.jamalang"
use IO from "https://raw.githubusercontent.com/Jamalam360/jamalang/main/std/io.jamalang"

fn add_or_subtract(a: UnsignedInt, b: UnsignedInt): UnsignedInt {
    if Random.boolean() {
        return a - b
    } else {
        return a + b
    }
}

# IO.prompt result is casted from `String` to `SignedInt`
inp1: SignedInt = IO.prompt("Enter a number: ")
inp2: SignedInt = IO.prompt("Enter a number: ")

IO.writeLine("Result: {add_or_subtract(inp1, inp2)}")
```

```
use IO from "https://raw.githubusercontent.com/Jamalam360/jamalang/main/std/io.jamalang"

glob = IO.prompt("Enter file glob: ")
IO.filesMatching(glob) -> (file) {
    file.delete()
    IO.writeLine("Deleted {file.name}")
}
```

```
use IO from "https://raw.githubusercontent.com/Jamalam360/jamalang/main/std/io.jamalang"

fn fib(UnsignedInt n): UnsignedInt {
    if n == 0 {
        return 0
    } else if n == 1 {
        return 1
    } else {
        return fib(n - 1) + fib(n - 2)
    }
}

n: UnsignedInt = IO.prompt("Enter a number: ")
IO.writeLine("Fibonacci of {n} is {fib(n)}")
```

```
# This is how the standard library will implement the `IO.write` and `IO.writeLine` functions.
# It uses the built-in method `write` and the built-in value `stdout` to write each character in the string to the stdout.
# `write` will also be usable for things like files.

fn write(String str) {
    str.chars() -> (char) {
        write(stdout, char)
    }
}

fn writeLine(String str) {
    write(str)
    write(0x0A) # 0x0A is the Unicode codepoint for the newline character, implicitly casted to a char
}
```
