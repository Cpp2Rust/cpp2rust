# Lambdas

A lambda becomes a struct with one field per capture, an inherent
`operator_call` method holding the translated body, a `Callable` impl so generic
code can invoke it, and, for a capture-less lambda, a `to_free_function`
associated method that yields it as a function pointer. Given

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
pub unsafe fn apply_0(mut fn_: lambda_1, mut x: i32) -> i32 {
    return (unsafe { lambda_1::operator_call(&fn_, x) });
}
unsafe fn main_0() -> i32 {
    let mut base: i32 = 10;
    let mut add_base: lambda_1 = (lambda_1 { base: &mut base });
    return (unsafe { apply_0(add_base, 5) });
}
pub struct lambda_1 {
    base: *mut i32,
}
impl lambda_1 {
    pub unsafe fn operator_call(&self, mut x: i32) -> i32 {
        return ((x) + (*self.base));
    }
}
impl Callable1<i32, i32> for lambda_1 {
    fn call(&self, a1: i32) -> i32 {
        unsafe { lambda_1::operator_call(self, a1) }
    }
}
```

and the refcount model produces

```rust
pub fn apply_0(fn_: lambda_1, x: i32) -> i32 {
    let fn_: Value<lambda_1> = Rc::new(RefCell::new(fn_));
    let x: Value<i32> = Rc::new(RefCell::new(x));
    return ({ lambda_1::operator_call(&(*fn_.borrow_mut()), (*x.borrow())) });
}
fn main_0() -> i32 {
    let base: Value<i32> = Rc::new(RefCell::new(10));
    let add_base: Value<lambda_1> = Rc::new(RefCell::new(
        (lambda_1 {
            base: base.as_pointer(),
        }),
    ));
    return ({ apply_0((*add_base.borrow()).clone(), 5) });
}
pub struct lambda_1 {
    base: Ptr<i32>,
}
impl lambda_1 {
    pub fn operator_call(&self, x: i32) -> i32 {
        let x: Value<i32> = Rc::new(RefCell::new(x));
        return ((*x.borrow()) + (self.base.read()));
    }
}
impl Callable1<i32, i32> for lambda_1 {
    fn call(&self, a1: i32) -> i32 {
        { lambda_1::operator_call(self, a1) }
    }
}
```

## Closure struct

The closure type is named `lambda_N`, numbered in order of appearance, and is
emitted at file scope. Each capture becomes a field named after the captured
variable, typed as the capture field of clang's closure class:

| Capture  | C++ field type | Unsafe        | Refcount        |
| -------- | -------------- | ------------- | --------------- |
| `[x]`    | `T`            | `T`           | `Value<T>`      |
| `[&x]`   | `T&`           | `*mut T`      | `Ptr<T>`        |
| `[&arr]` | `T (&)[N]`     | `*mut [T; N]` | `Ptr<Box<[T]>>` |
| `[this]` | `S*`           | `*mut S`      | `Value<Ptr<S>>` |

The struct follows the same trait rules as an ordinary struct, described in
[Traits](./traits.md).

The lambda expression itself becomes a struct literal. A by-value capture copies
the variable at that point, a by-reference capture takes its address, so C++'s
distinction between `[x]` and `[&x]` is preserved: the first never sees later
writes to `x`, the second does.

## Captures

Explicit, implicit and init-captures all translate the same way, because clang
materializes every capture as a closure field before the converter runs. `[=]`
and `[&]` produce one field per variable the body mentions, and `[y = x + 1]`
produces a field `y` whose value in the struct literal is the initializer
expression, evaluated where the lambda expression appears.

A captured `this` becomes a field named `this_`. A use of `this` in the body,
explicit or implied by a member access, reads that field: `self.this_` in the
unsafe model, `(*self.this_.borrow())` in the refcount model. From there member
access and method calls proceed as through any other pointer to the enclosing
class.

## Call operator

The body is emitted as `operator_call` on the closure struct. Its receiver
follows the C++ call operator:

- `&self` for an ordinary lambda, whose call operator is `const`;
- `&mut self` for a `mutable` lambda, so writes to by-value captures persist
  across calls;
- no receiver for a capture-less lambda, which makes `operator_call` a plain
  associated function.

## Callable

A lambda whose call operator is `const` also implements
[`Callable`](../../runtime/callable.md), so it can be passed to rules and
helpers that take a callable argument.

## Conversion to function pointer

A capture-less lambda has a conversion operator to function pointer. It becomes
a `to_free_function` method that returns the call operator as a function pointer
value: `Some(lambda_N::operator_call)` in the unsafe model and
`FnPtr::new(lambda_N::operator_call)` in the refcount model (see
[Function Pointers](./fn-pointers.md)). Assigning or passing such a lambda where
a function pointer is expected calls `to_free_function` on the closure object.

## Where the struct is emitted

The struct and its impls are hoisted to file scope after the enclosing top-level
declaration, so the function body keeps only the struct literal instead of
several items of boilerplate.
