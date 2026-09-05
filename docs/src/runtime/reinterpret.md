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

A C struct is translated as a Rust struct whose fields are `Value`s, and the
code generator emits the `ByteRepr` implementation next to it:

```c
struct header {
  int tag;
  int size;
};
```

```rust
pub struct header {
    pub tag: Value<i32>,
    pub size: Value<i32>,
}

impl ByteRepr for header {
    fn byte_size() -> usize {
        8
    }
    fn to_bytes(&self, buf: &mut [u8]) {
        (*self.tag.borrow()).to_bytes(&mut buf[0..4]);
        (*self.size.borrow()).to_bytes(&mut buf[4..8]);
    }
    fn from_bytes(buf: &[u8]) -> Self {
        Self {
            tag: Rc::new(RefCell::new(<i32>::from_bytes(&buf[0..4]))),
            size: Rc::new(RefCell::new(<i32>::from_bytes(&buf[4..8]))),
        }
    }
}
```

`byte_size` is `sizeof(struct header)`. `to_bytes` writes each field at its C
offset into an 8-byte buffer, so the buffer holds the struct exactly as it would
sit in C memory. `from_bytes` reads such a buffer back into a fresh struct.

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
// A view over p's allocation at byte offset 0, stepping by 1 byte.
let bytes: Ptr<u8> = p.reinterpret_cast::<u8>();

// Read: p.to_bytes() gives the 8 bytes, u8::from_bytes parses byte 0.
assert_eq!(bytes.read(), 0x01);
// Write: p.to_bytes(), replace byte 7 with 0xAA, u64::from_bytes back into p.
bytes.offset(7).write(0xAA);
// The write went into the original allocation.
assert_eq!(p.read(), 0xAA07060504030201);
```

A reinterpreted pointer counts its offset in bytes, so its arithmetic matches
the C cast exactly. Casting a view again does not stack views: the new pointer
keeps the handle to the original allocation.

Deleting through a reinterpreted pointer frees the original allocation. That is
how `free` works on a buffer that has been cast around: the pointer is
reinterpreted to bytes and the original allocation is deleted.

## Known limitations

Reading a struct through a reinterpreted pointer builds a fresh struct with
`from_bytes`, so its fields are new `Value`s that exist only as long as that
temporary struct. Three C patterns break because of this; all are set to be
fixed in the near future.

1. Taking the address of a field of a reinterpreted struct yields a pointer into
   the temporary, which dangles as soon as the temporary is dropped.
2. Writing to a field of a reinterpreted struct, translated as
   `p.upgrade().deref().field.borrow_mut()`, mutates the temporary returned by
   `p.upgrade().deref()` and never writes the bytes back to the original
   allocation, so the write is lost.
3. [Union accessors](../codegen/unions.md) return pointers to the union's
   storage; on a reinterpreted union that storage is the temporary, so the
   returned pointer dangles.

## AnyPtr casts

`AnyPtr::reinterpret_cast` first tries to recover the pointer as it was erased:
casting a `void *` back to the type it came from returns the original `Ptr<T>`,
with no byte view involved. Only a cast to a different type goes through the
byte representation:

```rust
let p: Ptr<u64> = Ptr::alloc(0x0807060504030201);
let any: AnyPtr = p.to_any();

// Same type as erased: the original Ptr<u64> comes back.
let back: Ptr<u64> = any.reinterpret_cast::<u64>();
assert!(back == p);

// Different type: a byte view over p's allocation, as with
// Ptr::reinterpret_cast.
let bytes: Ptr<u8> = any.reinterpret_cast::<u8>();
assert_eq!(bytes.read(), 0x01);
```
