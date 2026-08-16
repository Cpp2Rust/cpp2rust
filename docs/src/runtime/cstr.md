# C Strings

C and C++ strings are byte strings: programs manipulate individual bytes and the
contents need not be valid UTF-8, so strings are translated as `u8` buffers
rather than Rust `String` values. A string literal becomes a per-thread interned
buffer with a trailing zero byte, handed out as a `Ptr<u8>` by
`Ptr::from_string_literal`. `Ptr<u8>` also carries the memory functions C
strings rely on: `memcpy` (with `memmove` semantics for overlapping buffers
instead of undefined behavior), `memset`, `memcmp`, and `to_rust_string` for
crossing into Rust APIs.

`CStringIterator` walks the bytes of a `Ptr<u8>` up to the null terminator, and
`Display` for `Ptr<u8>` prints them, so a C string can be formatted directly.
`with_slice` and `with_slice_mut` expose a bounded byte range of the buffer as a
Rust slice for the duration of a closure, which is how a C buffer is passed to
Rust and nix functions such as `read` and `write`.
