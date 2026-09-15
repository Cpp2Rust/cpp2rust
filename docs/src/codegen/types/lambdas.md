# Lambdas

A lambda becomes a Rust closure with the same parameters and a translated body.
Given

```cpp
template <typename F> int apply(F fn, int x) { return fn(x); }

int main() {
  int base = 10;
  auto add_base = [&base](int x) { return x + base; };
  return apply(add_base, 5);
}
```

the unsafe model produces

```rust
pub unsafe fn apply_0(mut fn_: impl Fn(i32) -> i32, mut x: i32) -> i32 {
    return fn_(x);
}
unsafe fn main_0() -> i32 {
    let mut base: i32 = 10;
    return apply_0(
        (|x: i32| {
            return x + base;
        })
        .clone(),
        5,
    );
}
```

and the refcount model produces

```rust
pub fn apply_0(fn_: impl Fn(i32) -> i32, x: i32) -> i32 {
    let fn_: Value<_> = Rc::new(RefCell::new(fn_));
    let x: Value<i32> = Rc::new(RefCell::new(x));
    return (*fn_.borrow_mut())(*x.borrow());
}
fn main_0() -> i32 {
    let base: Value<i32> = Rc::new(RefCell::new(10));
    let add_base: Value<_> = Rc::new(RefCell::new(
        (|x: i32| {
            let x: Value<i32> = Rc::new(RefCell::new(x));
            return *x.borrow() + *base.borrow();
        }),
    ));
    return apply_0((*add_base.borrow()).clone(), 5);
}
```

## Closure and type

The closure lists the lambda's parameters with their translated types and
contains the body converted like a function body, including, in the refcount
model, the preamble that boxes each parameter. The lambda's own type is never
spelled: a variable holding one is `Value<_>` in the refcount model and the type
is inferred, and a function template parameter that receives one is
`impl Fn(A) -> R`, as `apply` shows. A call through such a parameter is a plain
call, `fn_(x)`, with the refcount model borrowing the boxed closure first.

## Captures

The C++ capture list is not translated. A Rust closure captures whatever it
mentions by reference, so `[&base]` and `[base]` produce the same closure and
both see the variable's current value at call time. For a by-reference capture
this is C++'s semantics; for a by-value capture it is not, since C++ copies the
variable when the lambda is created.

## Where the closure is emitted

The refcount model emits a variable initialized with a lambda as a boxed closure
once and clones it out of the box at each use.

> [!WARNING]
>
> The unsafe model does not emit a `let` for such a variable; the closure is
> emitted again at every use, which is why the example above shows it inline in
> the `apply_0` call. This was a workaround: a stored closure that captures
> locals by reference keeps them borrowed for as long as it lives, so
> `let foo = || { a += 1; a }; return foo() + a;` does not compile, while
> re-emitting the closure at each call keeps every borrow inside that call. It
> is a bug, since the lambda's creation and its uses are no longer the same
> object ([#314](https://github.com/Cpp2Rust/cpp2rust/issues/314)).

A capture-less lambda assigned to a function pointer becomes a function pointer
value: `Some(|...| ...)` in the unsafe model and `FnPtr::new(|...| ...)` in the
refcount model (see [Function Pointers](./fn-pointers.md)). Lambdas with
captures cannot be converted to function pointers, as in C++.
