# The Rule Preprocessors

Two build-time tools compile rule modules into the [JSON IR](./ir.md) that
`cpp2rust` loads at runtime. Both write into `<build>/rules/<module>/`:

* `cpp-rule-preprocessor` compiles `src.cpp`/`src.c` into `ir_src.json`.
* `rule-preprocessor` compiles `tgt_unsafe.rs`/`tgt_refcount.rs` into
  `ir_unsafe.json`/`ir_refcount.json`.

The C++ side is keyed by resolved callee signatures; the Rust side by rule
names. The two are joined by rule name when `cpp2rust` loads them.

## cpp-rule-preprocessor

A clang LibTooling executable (`cpp2rust/cpp_rule_preprocessor.cpp`) that runs
once per rule directory:

```bash
cpp-rule-preprocessor --dir rules/string --out <build>/rules/string/ir_src.json
```

Extra compiler flags can be passed with repeated `--cxxflags` options. CMake
invokes it for every rule module via the `preprocess-cpp-rules` target.

For each rule it:

1. Validates that every `fN` body is exactly one `return` statement.
2. Resolves the *callee* of the returned expression. For non-template rules
   this is just the called declaration. For template rules the callee is
   unresolved, so the tool instantiates the rule's template parameters with
   synthesized dummy types and runs overload resolution to find the function
   the rule refers to.
3. Prints the resolved declaration as a canonical signature string:
   `<return type> <qualified::name>(<param types>)[ const][ volatile][ &|&&]`.
   For `tN` aliases it prints the underlying type.

The output is a flat JSON object mapping rule names to these signature
strings.

Two details of the printer matter for matching. Typedefs that resolve to
builtin types are kept as written instead of being desugared: `size_t` prints
as `size_t`, not `unsigned long`, which is what lets it map to `usize` while
plain `unsigned long` maps to `u64`. And integer literals expanded from a
macro are recorded as the macro *name*, which is how
[constant rules](./writing-rules.md#enum-values-constants-and-macros) like the
`O_CREAT` one match by name.

## rule-preprocessor

A Rust binary crate built with the nightly toolchain because it links the
compiler's own libraries (`rustc_driver`, `rustc_middle`, ...). It processes
the whole rules tree in one invocation:

```bash
CARGO_TARGET_DIR=<target> cargo +nightly run --release \
    --manifest-path rule-preprocessor/Cargo.toml -- <build>/rules [rules-dir]
```

CMake first builds the `rules` crate (which also regenerates
`rules/src/modules.rs`) and then runs the preprocessor via the
`preprocess-rust-rules` target. It works in two phases.

**Phase 1, syntactic.** Each `tgt_*.rs` file is parsed with rust-analyzer's
parser, and functions whose `#[cfg]` does not match the host are dropped.
Every function body is then turned into a list of *fragments*, whose kinds
are described in
[The Rules IR](./ir.md#target-ir-ir_unsafejson--ir_refcountjson). The
fragmentation is mainly concerned with how the rule's arguments are used:
references to parameters and generics become placeholder and generic
fragments, while source text that does not involve an argument is kept
as-is.

Each placeholder is tagged with an *access*: read, write, or move. Some uses
give the access away syntactically (`&mut a0` is a write); those that do not,
typically method-call receivers and arguments, are left as `unknown` for
phase 2. This phase also applies the two
[preprocessor-side rewrites](./rewriting.md#preprocessor-side-rewrites) that
support rule rewriting.

**Phase 2, semantic.** The preprocessor compiles the `rules` crate in-process
with `rustc` and walks the typed HIR. This gives it the real signature of
every callee, which resolves the `unknown` accesses: passing to a
`&mut`/`*mut` parameter is a write, to a `&`/`*const` parameter a read, and
to `std::mem::take` a move. For type rules it also records which standard
traits (`Copy`, `Clone`, `Default`, ...) the mapped type implements. A
placeholder still `unknown` after this phase fails the build.

The result is one `ir_<model>.json` per input file, keyed by rule name.
