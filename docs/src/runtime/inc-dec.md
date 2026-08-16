# Increment and Decrement

C's `++` and `--` are expressions: `x++` yields the old value and `++x` the new
one, and both can appear inside a larger expression. Rust only has the `x += 1`
statement, so the `inc` and `dec` modules define one trait per operator form:

```rust
pub trait PostfixInc { fn postfix_inc(&mut self) -> Self; }
pub trait PrefixInc  { fn prefix_inc(&mut self) -> Self; }
pub trait PostfixDec { fn postfix_dec(&mut self) -> Self; }
pub trait PrefixDec  { fn prefix_dec(&mut self) -> Self; }
```

Each method updates the value in place and returns what the C expression
evaluates to: the postfix forms return a copy of the old value, the prefix forms
the new one.

```c
int x = 0;
while (x++ < 100 && x != 50) {
  ++x;
}
```

```rust
let x: Value<i32> = Rc::new(RefCell::new(0));
while (*x.borrow_mut()).postfix_inc() < 100 && *x.borrow() != 50 {
    (*x.borrow_mut()).prefix_inc();
}
```

The traits are implemented for the integer types with wrapping arithmetic, so
overflow behaves as C's unsigned wraparound and never panics, and for `f32` and
`f64`. [`Ptr<T>`](./rc.md#arithmetic) implements them by moving its offset one
element, and the map iterators by stepping to the neighbouring key. For each
translated enum the code generator emits `impl_enum_inc_dec!`, a macro exported
by `inc` that implements the four traits by converting through `i32`.

The unsafe model uses the same traits for integers and floats. For raw pointers
the same method names come from separate `Unsafe*` traits (`UnsafePrefixInc` and
so on), whose methods are `unsafe fn` and step the pointer with `offset(1)`, so
the generated code reads the same in both models:

```rust
let mut q: *mut i32 = p;
q.prefix_inc();
q.postfix_dec();
```
