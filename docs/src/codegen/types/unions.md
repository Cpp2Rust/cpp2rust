# Unions

Given

```c
union Number {
  int i;
  float f;
};

int foo(void) {
  union Number u;
  u.i = 42;
  return u.i;
}
```

the unsafe model produces

```rust
#[repr(C)]
#[derive(Copy, Clone)]
pub union Number {
    pub i: i32,
    pub f: f32,
}
impl Default for Number {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}
pub unsafe fn foo_0() -> i32 {
    let mut u: Number = <Number>::default();
    u.i = 42;
    return u.i;
}
```

and the refcount model produces

```rust
pub struct Number {
    __bytes: Value<Box<[u8]>>,
}
impl Number {
    pub fn i(&self) -> Ptr<i32> {
        (self.__bytes.as_pointer() as Ptr<u8>).reinterpret_cast()
    }
    pub fn f(&self) -> Ptr<f32> {
        (self.__bytes.as_pointer() as Ptr<u8>).reinterpret_cast()
    }
}
impl Default for Number {
    fn default() -> Self {
        Number {
            // 4 is sizeof(union Number): the size of its largest member
            __bytes: Rc::new(RefCell::new(Box::from([0u8; 4]))),
        }
    }
}
pub fn foo_0() -> i32 {
    let u: Value<Number> = Rc::new(RefCell::new(<Number>::default()));
    (*u.borrow_mut()).i().write(42);
    return (*u.borrow()).i().read();
}
```

(`Clone` and `ByteRepr` impls omitted.)

## Unsafe model

A union is a Rust `union` with `#[repr(C)]` and `#[derive(Copy, Clone)]`, one
field per member with the same types as a struct would have. Rust cannot derive
`Default` for a union, so a hand-written impl zeroes the bytes, which is also
what C's zero-initialization gives. Members are read and written like struct
fields; the whole function is `unsafe`, so no extra block is needed.

## Refcount model

Rust unions are unusable from safe code, so the refcount model stores the union
as one byte buffer, `__bytes: Value<Box<[u8]>>`, sized to the largest member,
and emits one accessor method per member. Each accessor takes a pointer to the
buffer and [reinterprets](../../runtime/reinterpret.md) it as the member type,
returning a `Ptr<T>` (a `Ptr` to the element type for array members). Member
access `u.i` therefore becomes a call, `u.i()`, and the read or write goes
through the pointer: `.read()` and `.write(v)` for scalars, `.upgrade().deref()`
for struct members whose fields are then accessed as usual.

Because every member views the same bytes, writing through one member and
reading through another has C's semantics: the bytes are reinterpreted, not
converted. This is also why the type must implement `ByteRepr`; the accessor's
`reinterpret_cast` needs the member types to have a byte-level representation.

`Default` fills the buffer with zeros, `Clone` copies the buffer into a fresh
`Value`, and `ByteRepr` copies the buffer in and out. The Rust struct has no
`pub` fields, so translated code can reach the storage only through the
accessors.

> [!WARNING]
>
> Accessors are broken on a reinterpreted union. Reading a `Ptr<U>` obtained
> from `reinterpret_cast` builds a temporary `U` from the bytes with
> `from_bytes`, so `p.upgrade().deref().i()` returns a pointer into that
> temporary's buffer, which dangles as soon as the statement ends, and a write
> through it would never reach the original allocation
> ([#311](https://github.com/Cpp2Rust/cpp2rust/issues/311)).
