# The Rule Preprocessors

Two build-time tools compile rule modules into the [Rules IR](./ir.md) that
`cpp2rust` loads at runtime. Both write into `<build>/rules/<module>/`:

- `cpp-rule-preprocessor` compiles `src.cpp`/`src.c` into `ir_src.json`.
- `rule-preprocessor` compiles `tgt_unsafe.rs`/`tgt_refcount.rs` into
  `ir_unsafe.json`/`ir_refcount.json`.

The C++ side is keyed by resolved callee signatures; the Rust side by rule
names. The two are joined by rule name when `cpp2rust` loads them.

## cpp-rule-preprocessor

A clang LibTooling executable (`cpp2rust/cpp_rule_preprocessor.cpp`) that runs
once per rule directory:

```bash
cpp-rule-preprocessor --dir rules/string --out <build>/rules/string/ir_src.json
```

Extra compiler flags can be passed with repeated `--cxxflags` options, though
CMake, which invokes the tool for every rule module via the
`preprocess-cpp-rules` target, passes none. Note that the parent directory of
`--out` must already exist; CMake creates it before each invocation, so a manual
run must do the same.

Rule sources are always compiled with the fixed flag set from
`cpp2rust/compat/platform_flags.h`, the same one used to parse input programs
(see [Compat Shims](./compat.md)). There is no compilation database and no
`-std=` flag: the language is chosen by clang from the file extension, and
`src.c` is processed before `src.cpp`.

For each rule it:

1. Validates that every `fN` body is exactly one `return` statement.
2. Resolves the _callee_ of the returned expression. For non-template rules this
   is just the called declaration. For template rules the callee is unresolved,
   so the tool instantiates the rule's template parameters with synthesized
   dummy types and runs overload resolution to find the function the rule refers
   to.
3. Prints the resolved declaration as a canonical signature string:
   `<return type> <qualified::name>(<param types>[, ...])[ const][ volatile][ &|&&]`,
   where `, ...` appears for C-variadic functions and the trailing qualifiers
   only for methods. For `tN` aliases it prints the underlying type.

The output is a flat JSON object mapping rule names to these signature strings.

The printer preserves typedef sugar instead of desugaring it: `size_t` prints as
`size_t`, not `unsigned long`, which is what lets it map to `usize` while plain
`unsigned long` maps to `u64` (for `tN` aliases this preservation is explicit;
inside function signatures the spelling survives through the printing policy).
Integer literals expanded from a macro are recorded as the macro _name_, which
is how [constant rules](./writing-rules.md#enum-values-constants-and-macros)
like the `O_CREAT` one match by name.

## rule-preprocessor

A Rust binary crate built with the nightly toolchain because it links the
compiler's own libraries (`rustc_driver`, `rustc_middle`, ...). It processes the
whole rules tree in one invocation:

```bash
CARGO_TARGET_DIR=<target> cargo +nightly run --release \
    --manifest-path rule-preprocessor/Cargo.toml -- <build>/rules [rules-dir]
```

The environment is load-bearing:

- `CARGO_TARGET_DIR` must be set (the tool aborts otherwise): the rlibs of the
  rule dependencies (`libcc2rs`, `libc`, `nix`, ...) are looked up in
  `$CARGO_TARGET_DIR/<profile>/deps`, which the `cargo run` above populates. The
  crate list is hardcoded, so a new dependency in `rules/Cargo.toml` also needs
  an entry in `rule-preprocessor/src/semantic.rs`.

  > [!WARNING] Stale rlibs from an earlier build can be picked up silently. Run
  > `ninja clean` to fix this.

- The sysroot comes from running `rustc --print=sysroot`, so the `rustc` on
  `PATH` must be the same nightly the preprocessor was built with (running
  through `cargo +nightly run` guarantees this).
- `rules-dir` is optional and defaults to the relative path `../rules`, resolved
  against the _current working directory_ of the process.

CMake drives all of this via the `preprocess-rust-rules` target: it first builds
the `rules` crate with the stable toolchain (which also regenerates
`rules/src/modules.rs`), then runs the preprocessor with
`CARGO_TARGET_DIR=<build>/target_preprocessor`. That initial `cargo build` of
the `rules` crate is what actually gates the build on rule bodies type-checking
(see below). The preprocessor works in two phases.

**Phase 1, syntactic.** Each `tgt_*.rs` file is parsed with rust-analyzer's
parser, and functions whose `#[cfg]` does not match the host are dropped. Every
function body is then turned into a list of _fragments_, whose kinds are
described in [The Rules IR](./ir.md#target-ir-ir_unsafejson--ir_refcountjson).
The fragmentation is mainly concerned with how the rule's arguments are used:
references to parameters and generics become placeholder and generic fragments,
while source text that does not involve an argument is kept as-is.

Each placeholder is tagged with an _access_: read, write, or move. Some uses
give the access away syntactically (`&mut a0` is a write); those that do not,
typically method-call receivers and arguments, are left as `unknown` for
phase 2. This phase also applies the two
[preprocessor-side rewrites](./rewriting.md#preprocessor-side-rewrites) that
support rule rewriting.

**Phase 2, semantic.** The preprocessor compiles the `rules` crate in-process
with `rustc` and walks the typed HIR. This gives it the real signature of every
callee, which resolves the `unknown` accesses: passing to a `&mut`/`*mut`
parameter is a write, to a `&`/`*const` parameter a read, and to
`std::mem::take` a move. For type rules it also records which of the nine
derivable standard traits (`Copy`, `Clone`, `Debug`, `Default`, `PartialEq`,
`Eq`, `PartialOrd`, `Ord`, `Hash`) the mapped type implements. A placeholder
still `unknown` after this phase fails the build.

The preprocessor assumes the `rules` crate is buildable, which the earlier
`cargo build` of the crate ensures; errors from the in-process compilation are
therefore only reported as a warning.

The result is one `ir_<model>.json` per input file, keyed by rule name. The
output file name is derived from the input file name (`tgt_unsafe.rs` becomes
`ir_unsafe.json`) and the module directory is the direct parent of the
`tgt_*.rs` file.
