# C Strings

C and C++ strings are byte strings: programs manipulate individual bytes and the
contents need not be valid UTF-8, so strings are translated as `u8` buffers
rather than Rust `String` values. A string literal becomes a per-thread interned
buffer with a trailing zero byte, handed out as a `Ptr<u8>` by
`Ptr::from_string_literal`. `Ptr<u8>` also carries the memory functions C
strings rely on: `memcpy` (with `memmove` semantics for overlapping buffers
instead of undefined behavior), `memset`, `memcmp`, and `to_rust_string` for
crossing into Rust APIs.

`CStringIterator`, returned by `to_c_string_iterator`, walks the bytes of a
`Ptr<u8>` up to the null terminator; the `string.h` rules are built on it, and
`Display` for `Ptr<u8>` prints it, so a C string can be formatted directly.

Copying a string does not go through the iterator, which would grow its
destination as it goes. `with_c_str` scans for the null terminator in the
backing storage and lends the bytes to a closure, so measuring or borrowing a
string allocates nothing (`c_str_len`, and `count` on a `CStringIterator`, are
built on it). `to_c_bytes` and `to_rust_string`, and through them `strdup` and
the `std::string` constructors, copy the string with a single allocation.
`to_c_bytes` leaves room for one more byte, which callers usually spend on the
terminator or a newline.

Iterating a `Ptr<T>` reports how many elements are left, so collecting a `Ptr`
into a `Vec` also allocates once.
