# void Pointers

`void *` is translated as `AnyPtr`, a type-erased `Ptr`. `to_any` erases the
element type and `reinterpret_cast` recovers it:

```c
char data[] = "hi";
void *vp = data;
char *cp = vp;
```

```rust
let data: Value<Box<[u8]>> = Rc::new(RefCell::new(Box::from(*b"hi\0")));
let vp: Value<AnyPtr> = Rc::new(RefCell::new((data.as_pointer() as Ptr<u8>).to_any()));
let cp: Value<Ptr<u8>> = Rc::new(RefCell::new((*vp.borrow()).reinterpret_cast::<u8>()));
```

`reinterpret_cast` returns the original pointer when the requested type matches
the erased one, and a [byte-level view](./reinterpret.md) otherwise.

The `malloc` family allocates and frees through `AnyPtr`, so the returned
pointer is cast to the requested type and cast back to free it:

```c
int *p = malloc(sizeof(int));
*p = 42;
free(p);
```

```rust
// malloc_refcount(n) is Ptr::alloc_array(vec![0u8; n].into_boxed_slice()).to_any()
let p: Value<Ptr<i32>> = Rc::new(RefCell::new(
    malloc_refcount(::std::mem::size_of::<i32>()).reinterpret_cast::<i32>(),
));
(*p.borrow()).write(42);
free_refcount((*p.borrow()).to_any());
```

`AnyPtr` also carries `memcpy`, `memset`, and `memcmp`, forwarding to the
`Ptr<u8>` versions from [C Strings](./cstr.md) over the byte view of its
pointee.

> [!WARNING]
>
> Two `AnyPtr` values are equal only when they were erased from the same pointer
> type and compare equal as that type. A `void *` obtained from a `Ptr<i32>` and
> one obtained from a `Ptr<u8>` into the same allocation compare unequal, where
> C would consider them the same address. This is set to be fixed by comparing
> through the byte view instead.

Casts between `AnyPtr` and integers use the same `to_int` and `from_int` as
[`Ptr<T>`](./rc.md#integer-casts).
