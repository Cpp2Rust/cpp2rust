# Overview

Proving ownership in the presence of C++'s unrestricted aliasing is undecidable
in general, so `cpp2rust` does not try to satisfy Rust's borrow checker
statically. Its default output, the refcount model, moves Rust's ownership and
mutability checks to run time: reference counting replaces static ownership, and
dynamic borrow checks replace static mutability checks. This trades some speed
for safety, and it lets every program be translated.

`libcc2rs` is the runtime library where those checks live: a small crate of
auxiliary types and functions, such as `Value<T>`, `Ptr<T>`, and `AnyPtr`, that
translated programs link against. Keeping this machinery in one library keeps
the generated refcount code free of `unsafe`.

Every Rust file `cpp2rust` emits imports the whole crate:

```rust
extern crate libcc2rs;
use libcc2rs::*;
```

## Module map

The modules fall into three groups.

The refcounted pointer model, the core of the refcount output:

- [`rc`](./rc.md): `Value<T>` and `Ptr<T>`, the refcounted stand-ins for C
  values and pointers.
- [`cstr`](./cstr.md): string literals, the `string.h` memory functions, and
  iteration over `Ptr<u8>` byte strings.
- [`void`](./void.md): `AnyPtr`, the type-erased pointer for `void *`.
- [`ptr_dyn`](./ptr-dyn.md): `PtrDyn<dyn T>`, pointers to virtual classes.
- [`reinterpret`](./reinterpret.md): the `ByteRepr` trait and allocation views
  that let a refcounted allocation be reinterpreted at the byte level, as C
  pointer casts do.
- [`alloc`](./rc.md#the-heap): `malloc`, `free`, `realloc`, and `calloc` over
  refcounted byte arrays.

Language-feature emulation, used by both models:

- [`inc` and `dec`](./inc-dec.md): traits implementing the four `++`/`--`
  operator forms.
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
