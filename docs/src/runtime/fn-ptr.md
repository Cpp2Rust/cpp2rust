# Function Pointers

A C function pointer can be null, compared for equality, cast to another
function pointer type and back, and stored in a `void *`. A Rust `fn` value can
be called and compared, but it is never null and its type is fixed, so the
refcount model translates function pointers as `FnPtr<T>`, where `T` is the Rust
`fn` type of the target:

```rust
pub struct FnPtr<T> { /* the function as first stored, and its current cast */ }

impl<T> FnPtr<T> {
    pub fn null() -> Self;
    pub fn new(f: T) -> Self;
    pub fn is_null(&self) -> bool;
    pub fn cast<U>(&self, adapter: Option<U>) -> FnPtr<U>;
    pub fn to_any(&self) -> AnyPtr;
}
```

`FnPtr` dereferences to the function, so a call through it is `(*fp)(args)`.
Calling a null pointer panics with `ub:`.

`FnPtr` stores its function type-erased and identifies it by address through
the `FnAddr` trait. Rust has no way to write an impl for every `fn` arity at
once, so `FnAddr` is implemented by a macro for `fn` types of zero to sixteen
parameters. A function with more parameters cannot be wrapped in an `FnPtr`,
and taking its address fails to compile with a missing `FnAddr` bound.

```cpp
typedef int (*int_fn)(int);
int double_it(int x) { return x * 2; }

int_fn fn = double_it;
int r = fn(5);
```

```rust
let fn_: Value<FnPtr<fn(i32) -> i32>> =
    Rc::new(RefCell::new(FnPtr::<fn(i32) -> i32>::new(double_it_0)));
let r: Value<i32> = Rc::new(RefCell::new((*(*fn_.borrow()))(5)));
```

## Casts

C code casts function pointers to a different type and calls through the new
type. When the two types are not compatible this is undefined behavior, but the
argument types involved usually have the same representation, so implementations
accept the call and programs rely on it. Below, `add_offset` takes an `int *`,
but is called through a pointer that takes a `void *`:

```c
typedef int (*generic_int_fn)(void *, int);
int add_offset(int *base, int offset) { return *base + offset; }

generic_int_fn gfn = (generic_int_fn)add_offset;
int result = gfn(&val, 42);
```

In Rust `fn(Ptr<i32>, i32) -> i32` and `fn(AnyPtr, i32) -> i32` are unrelated
types, so the code generator emits an adapter: a function of the target type
that converts the arguments and calls the original. `cast` stores it, and calls
through the cast pointer go through the adapter:

```rust
let gfn: Value<FnPtr<fn(AnyPtr, i32) -> i32>> = Rc::new(RefCell::new(
    FnPtr::<fn(Ptr<i32>, i32) -> i32>::new(add_offset_4).cast::<fn(AnyPtr, i32) -> i32>(Some(
        (|a0: AnyPtr, a1: i32| -> i32 { add_offset_4(a0.reinterpret_cast::<i32>(), a1) })
            as fn(AnyPtr, i32) -> i32,
    )),
));
let result: Value<i32> = Rc::new(RefCell::new((*(*gfn.borrow()))(val.as_pointer().to_any(), 42)));
```

The code generator can build an adapter when the arguments and return type of
the two function types have the same representation. Otherwise it passes `None`,
and calling through the cast pointer panics with `ub:`.

Equality compares the address of the function the pointer was created with.

Casting a function pointer to `void *` is `to_any`, and `AnyPtr::cast_fn::<T>`
recovers it. `reinterpret_cast` on an `AnyPtr` holding a function currently
panics, as do [integer casts](./rc.md#integer-casts) on a `Ptr`; both are set to
be fixed in the near future.

## The unsafe model

The unsafe model uses `Option<unsafe fn(...)>` directly, with `None` as the null
pointer, and casts between function pointer types with `std::mem::transmute`; it
does not use `FnPtr`.
