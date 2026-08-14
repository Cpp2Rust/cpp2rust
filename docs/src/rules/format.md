# Rule Format

A rule module is a directory under `rules/`, usually named after the header or
library it covers (`rules/unistd`, `rules/vector`, `rules/string`, ...). It
contains:

- `src.cpp` and/or `src.c`: the C++ (or C) side of each rule.
- `tgt_unsafe.rs`: the Rust targets for the unsafe model.
- `tgt_refcount.rs`: the Rust targets for the reference counting model
  (optional, see below).

A rule is a pair of same-named functions on the two sides. Names determine the
rule kind:

- `f1`, `f2`, ... are _expression rules_: they map a C++ call, member access,
  constructor, or constant to a Rust expression.
- `t1`, `t2`, ... are _type rules_: they map a C++ type to a Rust type.

## Expression rules

On the C++ side, an `fN` function must have a body that is exactly one `return`
statement. The returned expression is the pattern: the preprocessor resolves the
_callee_ of that expression (the function, method, constructor, enum constant,
or macro being used) and that becomes the rule's matching key. The function
parameters stand for the arguments at the call site.

```cpp
// rules/unistd/src.cpp
int f4(const char *pathname) { return unlink(pathname); }
```

On the Rust side, the same-named function gives the replacement expression.
Parameters must be named `a0`, `a1`, ... and correspond positionally to the C++
parameters:

```rust
// rules/unistd/tgt_unsafe.rs
unsafe fn f4(a0: *const libc::c_char) -> i32 {
    libc::unlink(a0)
}
```

```rust
// rules/unistd/tgt_refcount.rs
fn f4(a0: Ptr<u8>) -> i32 {
    match nix::unistd::unlink(a0.to_rust_string().as_str()) {
        Ok(()) => 0,
        Err(__e) => {
            libcc2rs::cpp2rust_errno().write(__e as i32);
            -1
        }
    }
}
```

When the converter encounters `unlink(x)` in the input, it emits the rule body
with the translated `x` substituted for `a0`.

## Type rules

On the C++ side, a `tN` rule is a type alias (`using` or `typedef`). On the Rust
side, it is a _zero-argument function_ whose return type is the mapped Rust type
and whose body is the default initializer for that type:

```cpp
// rules/vector/src.cpp
template <typename T1> using t1 = std::vector<T1>;
```

```rust
// rules/vector/tgt_unsafe.rs
fn t1<T1>() -> Vec<T1> {
    Vec::new()
}
```

## Model layering

The loader always reads `ir_unsafe.json` first. When translating with the
reference counting model, it then overlays `ir_refcount.json` on top: entries
with the same rule name replace the unsafe ones.

This means `tgt_refcount.rs` only needs to contain the rules that _differ_ from
the unsafe model. For example, the `__builtin_mul_overflow` rule in
`rules/builtin` has a pointer out-parameter (`a2` below), so the two models
translate it differently: in the unsafe model `a2` is a raw `*mut i64` written
through a deref, while in the refcount model it is a `Ptr<i64>` written through
`Ptr::write`. The other two arguments are identical in both models:

```rust
// rules/builtin/tgt_unsafe.rs
unsafe fn f9(a0: i64, a1: i64, a2: *mut i64) -> bool {
    let (val, ovf) = a0.overflowing_mul(a1);
    *a2 = val;
    ovf
}
```

```rust
// rules/builtin/tgt_refcount.rs
fn f9(a0: i64, a1: i64, a2: Ptr<i64>) -> bool {
    let (val, ovf) = a0.overflowing_mul(a1);
    a2.write(val);
    ovf
}
```

The module's other rules (byte swaps, `__builtin_expect`, ...) translate
identically in both models, so they appear only in `tgt_unsafe.rs` and the
refcount model inherits them. A module where no rule needs a refcount-specific
translation can omit `tgt_refcount.rs` entirely.

## C and C++ sources

A module may have both `src.c` and `src.cpp`; both are preprocessed and merged
into one `ir_src.json`. Defining the same rule name in both files is a hard
error, so numbering must not collide.

This split is necessary because rules match on the exact canonical signature of
the callee, and some libc functions have _different signatures in C and C++_.
For example, C has a single `char *strchr(const char *, int)`, while C++
replaces it with const-correct overloads such as
`const char *strchr(const char *, int)`. Since the signatures differ,
`rules/cstring` defines one rule per language:

```cpp
// rules/cstring/src.cpp
const char *f6(const char *a0, int a1) { return strchr(a0, a1); }
```

```c
// rules/cstring/src.c
char *f5(const char *a0, int a1) { return (strchr)(a0, a1); }
```

The C++ rule matches `strchr` calls in code translated as C++, the C rule
matches them in code translated as C.

`rules/builtin` uses this to cover both languages: `src.cpp` defines `f9`/`f10`
for the C++ `__builtin_mul_overflow` (returning `bool`) while `src.c` defines
`f12`/`f13` for the C version (returning `int`); their Rust bodies are
identical.

## The `rules` crate

The whole `rules/` tree is a single Rust crate. `rules/build.rs` walks the tree,
collects every `tgt_*.rs`, and generates `rules/src/modules.rs` with one
`#[path = ...]` module per file. Building the crate therefore type-checks every
rule body against the crates the rule targets call into, which are declared as
dependencies in `rules/Cargo.toml` (`libcc2rs`, `libc`, `nix`, ...). The Rust
rule preprocessor compiles exactly this crate to resolve types in rule bodies.
`rules/src/` is the only subdirectory that is not a rule module.
