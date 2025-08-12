# Rust ffi examples

## Introduction

## Examples

* [ffi_01_snappy_binding](#ffi_01_snappy_binding): call C from Rust
* [ffi_02_snappy_binding](#ffi_02_snappy_binding)
* [ffi_03_calling_rust_from_c](#ffi_03_calling_rust_from_c): call a basic Rust function from C
* [ffi_04_rust_callbacks](#ffi_04_rust_callbacks)
* [ffi_05_rust_callbacks_02](#ffi_05_rust_callbacks_02): register and call a Rust function as a callback in C
* [ffi_06_foreign_globals](#ffi_06_foreign_globals): access global variables in C from Rust
* [ffi_07_variadic_functions](#ffi_07_variadic_functions): call a C variadic function from Rust 
* [ffi_08_opaque_structs](#ffi_08_opaque_structs): Use an opaque struct (defined in Rust lib) in C

### ffi_01_snappy_binding

Call C function from Rust ([Nomicon ffi](https://doc.rust-lang.org/nomicon/ffi.html#calling-foreign-functions)).
This example is calling functions from the C library [snappy](https://github.com/google/snappy).

Setup: 
* sudo apt install pkg-config libsnappy-dev

Run:
* cargo run -p ffi_01_basic_snappy_binding

### ffi_02_snappy_binding

Same as rust_ffi_snappy_binding_01 but dealing with an enum as a return type.

### ffi_03_calling_rust_from_c

Call a basic Rust function from C (from [Nomicon ffi](https://doc.rust-lang.org/nomicon/ffi.html#calling-rust-code-from-c))

Compile Rust library:
* cargo build
* ll ../target/debug//libffi_03*

Compile C binary:

* cd resources
* gcc call_rust.c -o call_rust -lffi_03_calling_rust_from_c -L../../target/debug
* LD_LIBRARY_PATH=../../target/debug ./call_rust

### ffi_04_rust_callbacks

Register and call a Rust function as a callback (from [Nomicon ffi](https://doc.rust-lang.org/nomicon/ffi.html#callbacks-from-c-code-to-rust-functions))

Note: this is a simplified example (Calling Rust callbacks from C binary)

Compile Rust library:
* cargo build
* ll ../target/debug//libffi_04*

Compile C binary:

* cd resources
* gcc call_rust.c -o call_rust -lffi_04_rust_callbacks -L../../target/debug
* ./call_rust

### ffi_05_rust_callbacks_02

Same as ffi_04_rust_callbacks but:
* Using a struct `RustOjbect` in the callback function
* Use Rust code to call the C library that uses a Rust function as a callback

Compile C library:
* cd resources
* gcc -Wall -c extlib.c -o extlib.o
* ar rcs extlib.a extlib.o

Compile Rust:
* cargo build

### ffi_06_foreign_globals

Access global variables in C lib readline from Rust (from [Nomicon ffi](https://doc.rust-lang.org/nomicon/ffi.html#accessing-foreign-globals))

Setup:
* sudo apt install pkg-config libreadline-dev

* cargo run

### ffi_07_variadic_functions

Use a [C variadic function](https://en.cppreference.com/w/c/variadic.html) from Rust

* cargo run

### ffi_08_opaque_structs

Define an opaque struct in Rust lib (FileData) and use it in C.
Based on [rust ffi wrong way blog post](https://www.ralphminderhoud.com/blog/rust-ffi-wrong-way/)

Compile Rust library:
* cargo build
* ll ../target/debug//libffi_08*

Compile C binary:
* cd resources
* gcc -Wall api_1.c -o api_1 -lffi_08_opaque_structs -L../../target/debug 
* ./api_1

