# Function Pointers

Given

```cpp
typedef int (*op_t)(int);

int inc(int x) { return x + 1; }

op_t pick() { return inc; }

int apply(op_t f) {
  if (f == nullptr) {
    return 0;
  }
  return f(10);
}
```

the unsafe model produces

```rust
pub unsafe fn inc_0(mut x: i32) -> i32 {
    return x + 1;
}
pub unsafe fn pick_1() -> Option<unsafe fn(i32) -> i32> {
    return Some(inc_0);
}
pub unsafe fn apply_2(mut f: Option<unsafe fn(i32) -> i32>) -> i32 {
    if f.is_none() {
        return 0;
    }
    return f.unwrap()(10);
}
```

and the refcount model produces

```rust
pub fn inc_0(x: i32) -> i32 {
    let x: Value<i32> = Rc::new(RefCell::new(x));
    return *x.borrow() + 1;
}
pub fn pick_1() -> FnPtr<fn(i32) -> i32> {
    return FnPtr::<fn(i32) -> i32>::new(inc_0);
}
pub fn apply_2(f: FnPtr<fn(i32) -> i32>) -> i32 {
    let f: Value<FnPtr<fn(i32) -> i32>> = Rc::new(RefCell::new(f));
    if (*f.borrow()).is_null() {
        return 0;
    }
    return (*(*f.borrow()))(10);
}
```

## Unsafe model

A function pointer is `Option<unsafe fn(A) -> R>`: `Option` because it can be
null, `unsafe fn` because every translated function is `unsafe`. Naming a
function where a pointer is expected wraps it in `Some(...)`, the null pointer
is `None`, a null check is `is_none()`, and a call is `f.unwrap()(args)`.
Function pointers are `Copy` and compare with `==` on the function's address.

## Refcount model

A function pointer is [`FnPtr<fn(A) -> R>`](../../runtime/fn-ptr.md), built with
`FnPtr::<fn(A) -> R>::new(f)`. `FnPtr` dereferences to the function, so a call
is `(*f)(args)`; the null pointer is `FnPtr::null()` and the check is
`is_null()`. `FnPtr` is not `Copy`, so storing or passing one that already lives
in a variable clones it, and equality compares the address of the wrapped
function, so a pointer stays equal to itself after being cast.

A capture-less lambda assigned to a function pointer becomes
`FnPtr::new(|...| ...)` with the closure inline (see [Lambdas](./lambdas.md)).
