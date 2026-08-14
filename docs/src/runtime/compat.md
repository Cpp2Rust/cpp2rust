# Compat Helpers

Some C interfaces are macros or platform-specific symbols rather than plain
functions. On the source side, `cpp2rust` rewrites them into ordinary calls (see
[Compat Shims](../rules/compat.md)); the `compat` module is the runtime side of
that rewrite.

`errno` expands to a platform-specific function call (`__errno_location` on
Linux, `__error` on macOS).

In the unsafe model, `cpp2rust_errno_unsafe` binds both platform symbols under
one name and returns the real libc `errno` location:

```rust
pub unsafe fn cpp2rust_errno_unsafe() -> *mut i32;
```

In the refcount model, `errno` is a thread-local refcounted `i32` that the
runtime maintains itself:

```rust
pub fn cpp2rust_errno() -> Ptr<i32>;
```

Refcount code reaches the operating system through the libc shims and nix, so
libc's `errno` is never read by this model. Instead, each rule is responsible
for writing the error code of a failed call into the refcounted value. Nothing
enforces this on the rule side, but a rule that skips the write breaks programs
that check `errno`, so it is part of writing a correct rule.

`malloc_usable_size` is bound under one name for both platforms (the symbol is
`malloc_size` on macOS).
