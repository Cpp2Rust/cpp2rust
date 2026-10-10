# Lambdas

A lambda becomes an [`FnPtr`](../../runtime/fn-ptr.md) in both models. Its type
is `FnPtr<fn(A) -> R>`, with the signature of the lambda's call operator, so it
can be written wherever C++ names the closure type: a variable, a struct field,
a parameter of an instantiated template, or `decltype`. A call is
`f.call(args)`.

Given

```cpp
int total = 0;
auto accumulate = [total](int x) mutable {
  total += x;
  return total;
};
accumulate(1);
```

the unsafe model produces

```rust
let mut total: i32 = 0;
let mut accumulate: FnPtr<fn(i32) -> i32> = lambda_unsafe!(
    {
        let total: i32 = total;
    },
    |x: i32| -> i32 {
        total += x;
        return total;
    }
);
(unsafe { accumulate.call(1) });
```

and the refcount model produces

```rust
let total: Value<i32> = Rc::new(RefCell::new(0));
let accumulate: Value<FnPtr<fn(i32) -> i32>> = Rc::new(RefCell::new(lambda!(
    {
        let total: Value<i32> = Rc::new(RefCell::new((*total.borrow())));
    },
    |x: i32| -> i32 {
        let x: Value<i32> = Rc::new(RefCell::new(x));
        (*total.borrow_mut()) += (*x.borrow());
        return (*total.borrow());
    }
)));
({ (*accumulate.borrow()).call(1) });
```

## The lambda macros

A lambda with captures is written with `lambda!` in the refcount model and
`lambda_unsafe!` in the unsafe model. Both take two arguments:

- a block with one `let` per capture, giving its name, its type and the value it
  is initialized with when the lambda is created;
- a closure with the lambda's parameters, its return type and the translated
  body.

The macro declares a hidden struct with one field per capture, and makes the
body the `call` method of that struct. The body is written with the names of the
captures, as the C++ body is, so the macro adds `self.` in front of each capture
name in it, which makes the name refer to the field of the struct. The `lambda!`
above expands to

```rust
{
    struct __Lambda {
        total: Value<i32>,
    }
    impl __Lambda {
        fn call(&self, x: i32) -> i32 {
            let x: Value<i32> = Rc::new(RefCell::new(x));
            (*self.total.borrow_mut()) += (*x.borrow());
            return (*self.total.borrow());
        }
    }
    FnPtr::<fn(i32) -> i32>::from_lambda(
        __Lambda {
            total: Rc::new(RefCell::new((*total.borrow()))),
        },
        __Lambda::call,
    )
}
```

The struct is local to the expression and its type is erased by the `FnPtr`, so
it never appears in the translated program. The initializers are not part of the
method: they are evaluated where the lambda is written, so `total` there is the
enclosing variable, while `total` in the body is `self.total`.

The two macros differ in how the method receives the struct. `lambda!` takes it
by immutable reference, since the captures of the refcount model are `Value`s
and are written through their cells. `lambda_unsafe!` takes it by mutable
reference, since the captures of the unsafe model are plain fields, and wraps
the body in `unsafe`.

Every use of a capture name in the body gets the `self.`, so the body cannot
give that name to something else: a variable declared in the body, or a
parameter of a closure in it, with the name of a capture is a compile error.

## Captures

The captures are the ones of the C++ closure type, which clang computes also for
`[=]` and `[&]`.

| Capture      | Unsafe model | Refcount model  |
| ------------ | ------------ | --------------- |
| `[x]`        | `T`          | `Value<T>`      |
| `[&x]`       | `*mut T`     | `Ptr<T>`        |
| `[n = expr]` | `T`          | `Value<T>`      |
| `[this]`     | `*mut S`     | `Value<Ptr<S>>` |
| `[*this]`    | `S`          | `Value<S>`      |

A capture by copy holds its own copy, made when the lambda is created, and keeps
its value between calls. A capture by reference holds a pointer to the variable
and is dereferenced in the body:

```cpp
int base = 10;
auto add_base = [&base](int x) { return x + base; };
```

```rust
let mut base: i32 = 10;
let mut add_base: FnPtr<fn(i32) -> i32> = lambda_unsafe!(
    {
        let base: *mut i32 = &mut base;
    },
    |x: i32| -> i32 {
        return ((x) + (*base));
    }
);
```

A captured `this` is the capture `this_`. Inside the body `this` refers to it,
and members are accessed through that pointer.

A constant that the body uses without capturing, such as a `const int` local
read by value, is replaced by its initializer.

## Lambdas without captures

A lambda without captures needs no struct and is a function pointer built from a
closure:

```cpp
auto one = [](int x) { return x + 1; };
```

```rust
let mut one: FnPtr<fn(i32) -> i32> = FnPtr::<fn(i32) -> i32>::new(|x: i32| -> i32 {
    unsafe {
        return ((x) + (1));
    }
});
```

Converting it to a function pointer gives, in the refcount model, the lambda
itself, which already has the type of the function pointer. The unsafe model
translates function pointers as `Option<unsafe fn(A) -> R>`, so the conversion
emits `Some(|...| ...)` with the closure again (see
[Function Pointers](./fn-pointers.md)). Lambdas with captures cannot be
converted to function pointers, as in C++.

A lambda without captures can be default-constructed since C++20. The
translation of `decltype(one) other;` emits the closure again, as do the fields
of that type in the `Default` of a struct.

## Limitations

- Copying a lambda does not copy its captures: the copy shares them with the
  original, so a `mutable` lambda and its copy update the same state, and the
  copy constructors of the captures do not run.
- Moving a lambda does not run the move constructors of its captures.
- Destroying a lambda does not run the destructors of its captures.
- A lambda that is default-constructed by a type translated with a
  [rule](../../rules/writing-rules.md) is wrong. The rule only has the type of
  the lambda, `FnPtr<fn(A) -> R>`, whose default is the null pointer and not the
  lambda, so calling it panics. This affects, for example,
  `std::set<int, decltype(cmp)> s;`, where the set constructs the comparator.
- Generic lambdas, whose call operator is a template, are not translated.
- A default-constructed lambda whose body declares a `static` local is rejected,
  as each emitted closure would get its own copy of the variable.
- The parameter and return types of a lambda must implement `FnPtrArg`, like
  those of any `FnPtr`.
