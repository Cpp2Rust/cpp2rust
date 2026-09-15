# Traits

Every emitted struct comes with a fixed set of trait implementations. Some are
derived, some are written out; which is which depends on the model. Enums derive
`Clone, Copy, PartialEq, Debug, Default` in both models, and unions derive
`Copy, Clone` in the unsafe model regardless of their fields; the table below is
for structs.

| Trait                                  | Unsafe model                                                          | Refcount model                                      |
| -------------------------------------- | --------------------------------------------------------------------- | --------------------------------------------------- |
| `Copy`                                 | derived when every field is copyable                                  | never (a `Value` is not `Copy`)                     |
| `Clone`                                | derived                                                               | hand-written, deep-copies each field into a new box |
| `Default`                              | derived when possible, else hand-written                              | same                                                |
| `Drop`                                 | not emitted ([#310](https://github.com/Cpp2Rust/cpp2rust/issues/310)) | hand-written from a user destructor with a body     |
| `Ord`, `PartialOrd`, `PartialEq`, `Eq` | hand-written from `operator<`                                         | same                                                |
| `ByteRepr`                             | not needed                                                            | hand-written for every record and enum              |

## Copy and Clone

In the unsafe model a record derives `Copy` unless a field is translated to a
`Vec`, `BTreeMap`, `Option<Box<T>>` (`std::unique_ptr`), or a record that is not
itself `Copy`; `Clone` is derived unless the C++ copy constructor is deleted.
The refcount model skips `Clone` altogether when the copy constructor is
deleted. The refcount model cannot derive either, because a `Value<T>` field is
an `Rc` and a derived `Clone` would only bump its count, leaving the copy
aliasing the original. The generated `Clone` therefore rebuilds every field:

```rust
impl Clone for Counter {
    fn clone(&self) -> Self {
        let mut this = Self {
            count_: Rc::new(RefCell::new(*self.count_.borrow())),
        };
        this
    }
}
```

This is what gives struct assignment and pass-by-value C++'s member-by-member
copy.

## Default

`Default` is the value of a `T x;` without initializer, of `T x = {}`, and of
the elements of `new T[n]`. It is derived when the derived impl gives the C zero
value, and hand-written otherwise: when the class has a user-defined default
constructor, `default()` calls it; when a field is a C array, a `std::array`, a
function pointer, or a libc record, `default()` builds the struct field by
field, each with the same default value the converter uses for a variable of
that type declared without an initializer:

```rust
impl Default for S {
    fn default() -> Self {
        S {
            head: 0_i32,
            tail: [0_i32; 3],
            buf: [0 as libc::c_char; 4],
        }
    }
}
```

Unions always get a hand-written impl that zeroes their bytes.

## Drop

A user-defined destructor with a non-empty body becomes `impl Drop`, with the
body translated as a method body. Only the refcount model emits it; the unsafe
model drops destructors silently
([#310](https://github.com/Cpp2Rust/cpp2rust/issues/310)).

## Comparison

A class that defines `operator<` (as a method or an out-of-line function) gets
`Ord`, `PartialOrd`, `PartialEq`, and `Eq`, all expressed through the emitted
`lt` method: `cmp` calls it both ways to pick `Less`, `Greater`, or `Equal`, and
`eq` is "neither is less". Only one comparison operator per class is supported,
and only `operator<`. The converter assumes the operator is `const`, which Rust
requires (`cmp` and `eq` take `&self`) but C++ does not; a non-`const`
`operator<` is still emitted as `lt(&self, ...)`.

## ByteRepr

The refcount model emits [`ByteRepr`](../../runtime/reinterpret.md#byterepr) for
every record and enum: `byte_size`, `to_bytes`, and `from_bytes` laid out with
the C offsets of the fields (enums go through their `i32` value). It is what
lets a `Ptr` to the type be reinterpreted as bytes, and bytes be read back as
the type. A record with a field that has no byte representation gets an empty
impl, and reinterpreting it panics at run time.
