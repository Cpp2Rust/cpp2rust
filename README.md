Cpp2Rust
========

Cpp2Rust translates C++ to fully safe Rust automatically. It is a syntax-driven
translator based on clang's AST.

Cpp2Rust's algorithm is described in the paper
[Cpp2Rust: Automatic Translation of C++ to Safe Rust](https://web.ist.utl.pt/nuno.lopes/pubs/cpp2rust-pldi26.pdf)
published at PLDI 2026.


## Overview

Cpp2Rust first parses the input C++ file(s) with clang and produces an AST.
It then traverses the AST and emits Rust code as strings, inserting
calls to the `libcc2rs` runtime library where needed (e.g., for raw pointer
semantics).
Finally, the Rust code is pretty-printed using `rustfmt` to a single `.rs` file.

By default the *reference counting model* is used, which produces fully safe
Rust.
A generator of unsafe Rust is also available through the `--model=unsafe`
command line argument for debugging and performance comparisons.

### Runtime library (`libcc2rs`)

The generated code relies on a runtime library designed to simplify the
translation process.
C pointers are converted into the `Ptr<T>` type provided by `libcc2rs`.
`Ptr<T>` models C pointer semantics, including null, arithmetic, and aliasing,
while satisfying Rust's borrow checker through checked run-time operations.


## Requirements

On Ubuntu, install the required dependencies with:

```bash
sudo apt install libclang-22-dev clang++-22 ninja-build cmake
curl -LsSf https://astral.sh/ruff/install.sh | sh
```


## Build

```bash
mkdir build
cd build
cmake -GNinja ..
ninja
ninja check
```


## Run

### Translate a single file

```bash
./build/cpp2rust/cpp2rust --file=<file>.cpp -o=<file>.rs
```

By default, the reference couting model is used (fully safe output).
To generate unsafe Rust instead:

```bash
./build/cpp2rust/cpp2rust --file=<file>.cpp -o=<file>.rs --model=unsafe
```

**Minimal example.** Given `hello.cpp`:

```cpp
#include <cstdio>
int main() {
  printf("hello world\n");
  return 0;
}
```

Running `./build/cpp2rust/cpp2rust --file=hello.cpp -o=hello.rs` produces:

```rust
pub fn main() {
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    println!("hello world");
    return 0;
}
```

Compile and run with:

```bash
rustc hello.rs -L ../libcc2rs/target/debug
./hello
```

### Translate a whole program

First generate a
[`compile_commands.json`](https://clang.llvm.org/docs/JSONCompilationDatabase.html)
for your project. With CMake this is one extra flag:

```bash
cmake -DCMAKE_EXPORT_COMPILE_COMMANDS=ON ..
```

Then run:

```bash
./build/cpp2rust/cpp2rust --dir=<dir> -o <output>.rs
```

`<dir>` must be the directory that contains `compile_commands.json`.


## Test Suite

```bash
# Run all tests except benchmarks
ninja check

# Run only the unit tests
ninja check-unit

# Run libcc2rs unit tests
ninja check-libcc2rs

# Run benchmarks (compile & execute)
ninja check-benchmarks

# Check benchmark output without executing binaries
SKIP_RUN=1 ninja check-benchmarks

# Regenerate and check if the IR for the rules changed
ninja check-rules

# Regenerate expected output for unit tests after intentional changes
REPLACE_EXPECTED=1 ninja check-unit
```
