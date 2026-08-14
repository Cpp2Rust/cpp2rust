# Overview

This part of the book documents the internals of the code generator: how the
clang AST is traversed and how Rust code is emitted.

TODO: document the converter plugin mechanism (`cpp2rust/converter/plugins/`),
which intercepts constructs ahead of the translation rules (currently
`emplace_back`).
