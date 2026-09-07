# Introduction

Cpp2Rust translates C++ to fully safe Rust automatically. It is a syntax-driven
translator based on clang's AST.

Cpp2Rust's algorithm is described in the paper
[Cpp2Rust: Automatic Translation of C++ to Safe Rust](https://web.ist.utl.pt/nuno.lopes/pubs/cpp2rust-pldi26.pdf)
published at PLDI 2026.

## Overview

Cpp2Rust first parses the input C++ file(s) with clang and produces an AST. It
then traverses the AST and emits Rust code as strings, inserting calls to the
`libcc2rs` runtime library where needed (e.g., for raw pointer semantics).
Finally, the Rust code is pretty-printed using `rustfmt` to a single `.rs` file.

By default the _reference counting model_ is used, which produces fully safe
Rust. A generator of unsafe Rust is also available through the `--model=unsafe`
command line argument for debugging and performance comparisons.

## Runtime library (`libcc2rs`)

The generated code relies on a runtime library designed to simplify the
translation process. C pointers are converted into the `Ptr<T>` type provided by
`libcc2rs`. `Ptr<T>` models C pointer semantics, including null, arithmetic, and
aliasing, while satisfying Rust's borrow checker through checked run-time
operations.
