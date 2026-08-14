# Conventions

Most of these conventions are enforced by the preprocessors, and violating them
fails the build; the notes below call out the ones that are not checked.

## Naming

| Element               | C++ side                                      | Rust side                                           |
| --------------------- | --------------------------------------------- | --------------------------------------------------- |
| Expression rule       | `f1`, `f2`, ...                               | same name                                           |
| Type rule             | `t1`, `t2`, ... via `using`/`typedef`         | `fn tN() -> RustType` with no arguments             |
| Parameters            | free-form (`o`, `it`, `key`, `dst`, `n`, ...) | must be `a0`, `a1`, ... consecutive from 0          |
| Generics              | `T1`, `T2`, ... (type and non-type params)    | `T1`, `T2`, ... consecutive from 1                  |
| Variadic pack         | `typename... Args`                            | trailing `va: &[VaArg]`                             |
| Locals in Rust bodies |                                               | double-underscore prefix: `__v`, `__fd`, `__e`, ... |

Notes:

- Rule numbering is per module, and gaps are currently allowed (e.g. `rules/map`
  has no `f4`), though this might change in the future. Names must be unique
  across `src.c` and `src.cpp` combined.
- On the C++ side parameter names are free, but the _order_ defines the
  placeholder indices: the first parameter is `a0` on the Rust side, the second
  is `a1`, and so on. The receiver of a method rule is always the first
  parameter, hence `a0`.
- Generic parameters are matched positionally between the two sides, so `T1` in
  the Rust target means "whatever bound to `T1` in the C++ pattern".
- Locals introduced inside Rust rule bodies use a `__` prefix. This is not
  checked by the build, but it is needed: rule bodies are spliced inline into
  the generated code, so an unprefixed local could collide with a variable name
  from the translated program.

## Function qualifiers

- In `tgt_unsafe.rs`, expression rules are `unsafe fn`; type rules (`tN`) are
  plain `fn`.
- In `tgt_refcount.rs`, all rules are safe `fn`. The refcount model produces
  fully safe Rust, so a refcount rule body must not need `unsafe`.

The build does not check the qualifiers themselves; only rustc's usual rules
apply when the `rules` crate compiles. In particular, nothing stops an `unsafe`
block inside a refcount rule body from being spliced into the output, so keeping
refcount rules safe is what upholds the model's safety guarantee.

## C++ pattern shape

- An `fN` body must be exactly one `return` statement. The preprocessor rejects
  anything else.
- `return` statements are not allowed inside Rust rule bodies; write the result
  as a tail expression instead.
- Exercise exactly one construct per rule. If an API has several overloads,
  write one rule per overload (including separate rules for `const T &` versus
  `T &&` parameters).

## Argument accesses

Every use of an `aN` parameter in a rule body is classified as a read, write, or
move by the [rule preprocessor](./preprocessors.md#rule-preprocessor). Passing
an argument by value counts as a read, not a move; the only way to record a move
is `std::mem::take(&mut aN)`.

## Type checking

All `tgt_*.rs` files are compiled as part of the `rules` crate, so a rule body
that does not type-check against `libcc2rs`, `libc`, `nix`, etc. breaks the
build. If a rule needs a new crate dependency, add it to `rules/Cargo.toml` and
to the hardcoded crate list in `rule-preprocessor/src/semantic.rs` (see
[The Rule Preprocessors](./preprocessors.md#rule-preprocessor)).
