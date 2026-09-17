# Callable

In C++ a function pointer, a functor (a user-defined struct with `operator()`)
and a lambda are all called the same way, `f(x)`, and a template parameter
accepts any of them. Rust has the `Fn` traits for that role, but only closures
and safe `fn` items implement them; a user struct or an `unsafe fn` cannot.
`libcc2rs` therefore provides its own call trait, one per arity:

```rust
pub trait Callable0<R> { fn call(&self) -> R; }
pub trait Callable1<A1, R> { fn call(&self, a1: A1) -> R; }
pub trait Callable2<A1, A2, R> { fn call(&self, a1: A1, a2: A2) -> R; }
pub trait Callable3<A1, A2, A3, R> { fn call(&self, a1: A1, a2: A2, a3: A3) -> R; }
```

Three kinds of value implement it:

- safe `fn` items, through a blanket impl for every type implementing the
  matching `Fn`;
- `unsafe fn` pointers, through a second blanket impl that wraps the call in an
  `unsafe` block;
- [translated lambdas](../codegen/types/lambdas.md), through a call to
  `operator_call`.

Code written against a `CallableK` bound invokes any of them through
`.call(..)`. This is how the STL algorithm rules take their comparators:

```rust
fn f6<T1: Ord + Clone, T2>(a0: Ptr<T1>, a1: Ptr<T1>, a2: T2)
where
    T2: Callable2<Ptr<T1>, Ptr<T1>, bool>,
{
    a0.sort_with_cmp(a1.get_offset(), |x, y| a2.call(x, y))
}
```

## Why not `Fn`

Implementing `Fn`, `FnMut` or `FnOnce` for a user type needs the
`unboxed_closures` and `fn_traits` features, which are
[nightly-only](https://github.com/rust-lang/rust/issues/29625).

## Notes

There is one trait per arity, `Callable0` to `Callable3`, rather than a single
trait over a tuple of arguments, so that a call is written `call(x, y)` and not
`call((x, y))`.

`call` takes `&self`, so only callables with a `const` call operator can
implement the trait. A `mutable` lambda, whose `operator_call` takes
`&mut self`, gets no `Callable` impl. This is a limitation of the trait
definition, not of the lambda translation.
