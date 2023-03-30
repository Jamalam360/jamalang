# Jamalang

Jamalang is a transpiled, statically typed langauge with powerful type
inference. While not strictly object-oriented due to it's lack of traditional
inheritance, it still provides composability through via it's constraint system.
Jamalang also contains a powerful FFI system, allowing for the use of existing
libraries for any language that can compile to LLVM IR/bitcode.

## Standard Library Philosophy

A lot of Jamalang's functionality is encapsulated in its standard library,
implemented in Rust. The standard library contains types such as `String` and
`List`. It is composed of a `core` module, which is automatically imported into
any program, and other modules that can optionally be imported.
