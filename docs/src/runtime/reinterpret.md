# Type Reinterpretation

C code reads the same memory at different types: a `long` is inspected byte by
byte through a `char *`, a byte buffer from `malloc` is used as an array of
structs, a `struct sockaddr_in` is passed where a `struct sockaddr` is expected.
In the refcount model there are no raw bytes to point at: values are typed Rust
data behind refcounted cells. The `reinterpret` module supplies the byte view
these programs expect.

## ByteRepr

`ByteRepr` gives a type its C byte representation:

```rust
pub trait ByteRepr: 'static {
    fn byte_size() -> usize;
    fn to_bytes(&self, buf: &mut [u8]);
    fn from_bytes(buf: &[u8]) -> Self;
}
```

The primitive types serialize to their native-endian bytes, matching what C sees
on the host. The [libc shims](./libc-shims.md#the-sockaddr-family) implement the
trait by hand with the byte layout of their C structs. Types with no meaningful
C layout, such as `std::fs::File` or `Vec<T>`, implement the trait with defaults
that panic, so reinterpreting one is caught at run time.

## Views over the original allocation

`reinterpret_cast` copies nothing. It produces a `Ptr` in the `Reinterpreted`
kind: a handle to the original allocation plus a byte offset, stepping by the
target type's size. A read serializes the overlapping elements of the original
into bytes and parses the target value out of them; a write is a
read-modify-write back into the original. The data always lives in the original
allocation, so writes through the original are visible through every view and
writes through a view are visible everywhere else:

```rust
let p: Ptr<u64> = Ptr::alloc(0x0807060504030201);
let bytes: Ptr<u8> = p.reinterpret_cast::<u8>();

assert_eq!(bytes.read(), 0x01);
bytes.offset(7).write(0xAA);
assert_eq!(p.read(), 0xAA07060504030201);
```

A reinterpreted pointer counts its offset in bytes, so its arithmetic matches
the C cast exactly. Casting a view again does not stack views: the new pointer
keeps the handle to the original allocation.

Deleting through a reinterpreted pointer frees the original allocation. That is
how `free` works on a buffer that has been cast around: the pointer is
reinterpreted to bytes and the original allocation is deleted.

## AnyPtr casts

`AnyPtr::reinterpret_cast` first tries to recover the pointer as it was erased:
casting a `void *` back to the type it came from returns the original `Ptr<T>`,
with no byte view involved. Only a cast to a different type goes through the
byte representation.
