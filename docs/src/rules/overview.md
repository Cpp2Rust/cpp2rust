# Overview

Translation rules describe how C++ library APIs are mapped to Rust. Each rule
module lives in the `rules/` directory and pairs a C++ source file (`src.cpp`)
with its Rust translation for each model (`tgt_unsafe.rs` and
`tgt_refcount.rs`).

Every rule is expressed as ordinary, compilable C++ and Rust source code, free
of anything platform dependent. Both sides are run through real compilers at
build time, so a rule that does not compile fails the build, and the
platform-specific spellings (for example `bool` canonicalizing to `_Bool`) are
derived by the compiler on the host rather than written by hand. The same rule
sources work on every platform `cpp2rust` builds on.

Rules go through a build-time compilation pipeline before `cpp2rust` can use
them:

1. You author a rule module: C++ patterns in `src.cpp` and Rust targets in
   `tgt_unsafe.rs` / `tgt_refcount.rs`.
2. At build time, two preprocessors compile the module into Rules IR under
   `<build>/rules/<module>/`: `cpp-rule-preprocessor` compiles the C++ side into
   `ir_src.json`, and `rule-preprocessor` compiles the Rust side into
   `ir_unsafe.json` and `ir_refcount.json`.
3. At startup, `cpp2rust` loads the Rules IR files and indexes the rules by the
   canonical signature of the C++ construct they match.

The rest of this part covers each stage:

- [Rule Format](./format.md): the files that make up a rule module and how the
  two models are layered.
- [Writing Rules](./writing-rules.md): how to write rules for functions,
  methods, operators, types, constants, and variadics.
- [Compat Shims](./compat.md): how macro-based libc APIs like `errno` and
  `FD_SET` are rewritten into matchable function calls.
- [Conventions](./conventions.md): naming and style conventions rule authors
  must follow.
- [The Rule Preprocessors](./preprocessors.md): the two build-time tools that
  compile rules to the Rules IR.
- [The Rules IR](./ir.md): the JSON format the preprocessors emit.
- [Loading and Matching](./loading.md): how `cpp2rust` loads the Rules IR and
  matches rules against the input AST.
- [The Matching Engine](./matching.md): how a candidate rule's signature is
  unified against the input.
- [Rule Rewriting](./rewriting.md): how rule bodies are adapted at application
  time, in particular the `with_mut` rewrite.
