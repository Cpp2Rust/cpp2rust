# Overview

`libcc2rs` is the runtime library that translated programs link against. Every
Rust file `cpp2rust` emits begins with the same import:

```rust
extern crate libcc2rs;
use libcc2rs::*;
```

The crate re-exports everything at its root, so this single glob import gives
the generated code access to the whole API without qualified paths. The build is
wired into CMake: the crate is compiled with Cargo into an `rlib`, and
translated programs are compiled against that `rlib` together with
`libcc2rs-macros`, the companion proc-macro crate.

## The two output models

The library serves both output models. The refcount model depends on it for its
entire pointer representation: refcounted values, pointers, and byte-level views
all come from the runtime. The unsafe model expresses pointers as raw Rust
pointers and calls libc directly (its output also imports `libc::*`), so it only
uses the runtime for constructs that raw pointers cannot express, such as
`goto`, `switch` fallthrough, and variadic calls.

Some libc functions exist in the crate as real named functions, such as
`fread_refcount` and `fread_unsafe`, because translated programs can take their
address. A [rule body](../rules/format.md) alone has nothing to take the address
of, so the runtime defines a function with the signature the model expects.

## Module map

The modules fall into three groups.

The refcounted pointer model, the core of the refcount output:

- [`rc`](./rc.md): `Value<T>`, `Ptr<T>`, and `AnyPtr`, the refcounted stand-ins
  for C values and pointers.
- [`reinterpret`](./reinterpret.md): the `ByteRepr` trait and allocation views
  that let a refcounted allocation be reinterpreted at the byte level, as C
  pointer casts do.
- `alloc`: `malloc`, `free`, `realloc`, and `calloc` over refcounted byte
  arrays.

Language-feature emulation, used by both models:

- `inc` and `dec`: traits implementing the four `++`/`--` operator forms.
- `iterators`: iteration for C++ containers that need stable iterators, with an
  implementation for both refcount and unsafe.
- `fn_ptr`: `FnPtr`, function pointers with C-style address identity.
- `va_args`: `VaArg` and `VaList`, the representation of variadic calls.
- The `goto`, `goto_block`, and `switch` proc macros, re-exported from
  `libcc2rs-macros`, which rewrite unstructured control flow into state
  machines.

The OS and libc surface:

- [`io`](./io.md): `CFile` streams, the standard streams, and read/write
  helpers.
- [`format`](./io.md#formatting): `printf`-style format string evaluation.
- [`fd`](./io.md#file-descriptors): a registry tying integer file descriptors to
  their owning objects.
- [`libc_shims`](./libc-shims.md): safe wrappers over libc APIs, one submodule
  per area (files, directories, sockets, name resolution, polling, terminal
  control, time, and so on).
- [`compat`](./compat.md): platform-specific definitions, such as the location
  of `errno` and `malloc_usable_size`.

## Dependencies

The crate has four dependencies:

- `libcc2rs-macros` provides the control-flow proc macros.
- `libc` and `nix` provide the raw and safe OS interfaces the shims wrap.
- `jiff` backs the time shims.
- `sprintf` backs `printf`-style formatting.
