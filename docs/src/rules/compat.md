# Compat Shims

Rule matching needs a resolvable callee. The preprocessor keys every expression
rule on the function, method, constructor, constant, or global that the
pattern's `return` expression resolves to, and the only macros it can record are
those that
[expand to an integer literal](./writing-rules.md#enum-values-constants-and-macros),
which match by macro name. Any other macro is invisible to the rule system: by
the time clang has built the AST, the macro is gone and only its expansion
remains.

That is a problem for a small set of libc APIs that are specified as macros over
platform internals:

- `errno` is an object-like macro; glibc expands it to `(*__errno_location())`,
  macOS to `(*__error())`.
- `assert` expands to a conditional that stringifies the condition and calls a
  platform-specific failure handler with file and line arguments.
- `FD_SET`, `FD_CLR`, `FD_ISSET`, and `FD_ZERO` expand to bit manipulation on
  the `fd_set` representation, through helpers that differ per platform.
- `ntohl`, `ntohs`, `htonl`, and `htons` expand to byte-swap builtins or to
  nothing at all, depending on endianness.

There is no stable, platform-independent callee here to key a rule on. The
_compat headers_ in `cpp2rust/compat/` fix this by rewriting each such macro
into a call to a synthetic, well-known function before matching happens.

## How the shims work

`cpp2rust/compat` is injected as a system include directory ahead of the
platform headers in every clang invocation the project makes: both when
`cpp2rust` parses the input program and when `cpp-rule-preprocessor` compiles
rule sources. The shared flag list lives in `cpp2rust/compat/platform_flags.h`
(`getPlatformClangBeginFlags`), and the directory path is baked in at build time
via the `COMPAT_INCLUDE_DIR` definition.

A shim header sits at the same relative path as the real header it shadows
(`errno.h`, `sys/select.h`, `arpa/inet.h`, ...), so an ordinary
`#include <errno.h>` finds the shim first. The header then:

1. pulls in the real platform header with `#include_next` (a GNU extension; the
   shared flags pass `-Wno-gnu-include-next` for it),
2. `#undef`s the macro,
3. declares a `cpp2rust_*` shim function,
4. redefines the macro to call the shim.

`cpp2rust/compat/errno.h` in full:

```c
#include_next <errno.h>

#undef errno

int *cpp2rust_errno(void);

#define errno (*cpp2rust_errno())
```

The redefinition keeps `errno` an lvalue by dereferencing the returned pointer,
so both reads and assignments like `errno = 0` still parse; what the matcher
sees in either case is a call to `int *cpp2rust_errno()`.

Because the input program and the rule sources are compiled with the same shim
headers, both sides canonicalize to the same signature, and an ordinary
[expression rule](./writing-rules.md) matches it:

```c
// rules/errno/src.c
#include <errno.h>

int *f1(void) { return cpp2rust_errno(); }
```

The shim functions are declared but never defined on the C side. They only exist
so that the callee resolves; translation replaces the call with the rule body,
so no C implementation is ever linked. Whatever the shim is supposed to _do_ is
supplied by the Rust targets:

```rust
// rules/errno/tgt_unsafe.rs
unsafe fn f1() -> *mut i32 {
    libcc2rs::cpp2rust_errno_unsafe()
}
```

```rust
// rules/errno/tgt_refcount.rs
fn f1() -> Ptr<i32> {
    libcc2rs::cpp2rust_errno()
}
```

In the unsafe model `libcc2rs::cpp2rust_errno_unsafe` wraps the real platform
errno location (`__errno_location` on Linux, `__error` on macOS). The refcount
model instead _virtualizes_ errno as a thread-local `Value<i32>` inside
`libcc2rs`; this is the same cell that other refcount rules write when they
translate a failing libc call into
`libcc2rs::cpp2rust_errno().write(__e as i32)`.

A rule pattern may spell either the macro or the shim directly; the two are
identical after expansion. `rules/errno` and `rules/assert` call the shim by
name, while `rules/arpa_inet` and `rules/select` write the macro form:

```c
// rules/select/src.cpp
void f2(int fd, fd_set *set) { return FD_SET(fd, set); }
```

## The current shims

| Header         | Macros                                    | Shim functions                  | Rules                                                                                   |
| -------------- | ----------------------------------------- | ------------------------------- | --------------------------------------------------------------------------------------- |
| `assert.h`     | `assert`                                  | `cpp2rust_assert_fail(bool)`    | `rules/assert` maps it to `assert!(a0)`                                                 |
| `errno.h`      | `errno`                                   | `cpp2rust_errno()`              | `rules/errno`, see above                                                                |
| `arpa/inet.h`  | `ntohl`, `ntohs`, `htonl`, `htons`        | `cpp2rust_ntohl(x)`, ...        | `rules/arpa_inet` maps them to `u32::from_be`, `u16::to_be`, ...                        |
| `sys/select.h` | `FD_SET`, `FD_CLR`, `FD_ISSET`, `FD_ZERO` | `cpp2rust_fd_set(fd, set)`, ... | `rules/select` maps them to `libc::FD_SET(...)` (unsafe) or `CFdSet` methods (refcount) |

Note how the shim also normalizes the _shape_ of the API. C's `assert` is a
macro precisely so it can stringify its condition and capture file and line; the
shim reduces it to a plain `void(bool)` function, and the Rust side regains the
diagnostics by mapping it to the `assert!` macro.

## Adding a new shim

To make another macro-based API matchable:

1. Create the header in `cpp2rust/compat/` at the same relative path as the
   platform header that defines the macro.
2. Follow the pattern above: `#include_next` the real header, `#undef` the
   macro, declare a `cpp2rust_<name>` function with the macro's effective
   signature, and redefine the macro to call it.
3. Write rules for the shim in a `rules/` module as for any other function,
   including the corresponding header in `src.c`/`src.cpp`.
4. If a model needs runtime support (as refcount errno does), implement it in
   `libcc2rs` and call it from the rule target.

Keep the shim's signature platform-independent; the whole point is that both
sides of every rule see one canonical declaration on every platform.

## Related normalization

The same shared flag list also passes `-D_FORTIFY_SOURCE=0`, which keeps glibc
from substituting fortified variants (`__printf_chk` and friends) for standard
calls. Like the shims, this ensures that calls in the input program resolve to
the standard declarations the rules are written against.
