# Variadic Functions

Rust has no `...` parameters and no `va_list`. A variadic C function is
translated as a function whose last parameter is a slice of `VaArg`, an enum
with one variant per kind of value C's default argument promotions can produce:

```rust
pub enum VaArg {
    Int(i32),
    UInt(u32),
    Long(i64),
    ULong(u64),
    Double(f64),
    RawPtr(*mut c_void),
    Ptr(AnyPtr),
}
```

At a call site every extra argument is converted with `.into()`, which performs
the promotions (`char` and `short` to `int`, `float` to `double`) and erases
pointers to `AnyPtr` in the refcount model or `*mut c_void` in the unsafe model.
Inside the function, `va_list` is a `VaList`, a cursor over the slice:
`va_start` becomes `VaList::new(__args)`, `va_arg(ap, T)` becomes
`ap.arg::<T>()`, `va_copy` is a plain copy of the cursor, and `va_end` is a
no-op:

```c
int sum(int count, ...) {
  va_list ap;
  va_start(ap, count);
  int total = 0;
  for (int i = 0; i < count; i++)
    total += va_arg(ap, int);
  va_end(ap);
  return total;
}

sum(3, 10, 20, 30);
```

```rust
pub fn sum_0(count: i32, __args: &[VaArg]) -> i32 {
    let ap: Value<VaList> = Rc::new(RefCell::new(VaList::default()));
    (*ap.borrow_mut()) = VaList::new(__args);
    let total: Value<i32> = Rc::new(RefCell::new(0));
    // ...
    (*total.borrow_mut()) += (*ap.borrow_mut()).arg::<i32>();
    // ...
}

sum_0(3, &[10.into(), 20.into(), 30.into()]);
```

`arg::<T>()` goes through the `VaArgGet` trait, implemented for the integer and
floating types, raw pointers, `Ptr<T>`, `AnyPtr`, and `FnPtr<T>`. Integer
variants convert freely among the integer types, as `va_arg` does with types of
the same rank; asking for a pointer where an integer was passed, or the reverse,
panics.

Variadic libc functions such as `printf` and `fcntl` are handled by
[variadic rules](../rules/writing-rules.md#variadic-functions), whose bodies
receive the same `&[VaArg]` slice; `format_c` in the
[format module](./io.md#formatting) consumes one to evaluate a format string.
