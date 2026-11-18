# Rule Rewriting

A rule body is written against idiomatic Rust types: a rule that mutates a
vector declares its parameter as `&mut Vec<T1>`. But in the refcount model the
call-site argument is usually a `Ptr<Vec<T1>>`, and a `Ptr`
[cannot produce a long-lived `&mut`](../codegen/pointers.md). Instead of forcing
every rule to handle pointers, the code generator _rewrites_ the rule body at
application time.

## The `with_mut` rewrite

`libcc2rs` provides

```rust
impl<T> Ptr<T> {
    pub fn with_mut<R>(&self, f: impl FnOnce(&mut T) -> R) -> R { ... }
}
```

which checks the pointer, borrows the pointee mutably, and runs the closure on
it (with an immutable sibling `Ptr::with`). The refcount converter uses it to
bridge the gap; the unsafe converter never rewrites and simply emits receiver
followed by body. The rewrite fires when all three hold:

1. The rule body fragment is a method call whose receiver contains a placeholder
   (the preprocessor splits every method call into receiver and body fragments
   precisely to enable this). If the receiver contains several placeholders, the
   first one is used.
2. The receiver placeholder's access is write or move, i.e. the method takes
   `&mut self` or the rule mutates the parameter. Read access does not need the
   rewrite, since a read can go through a [`StrongPtr`](../codegen/pointers.md)
   obtained with `Ptr::upgrade`, or through a `read()` copy.
3. The call-site argument is a pointer, or an expression of reference type
   (which includes an operator call returning a reference).

The rule's method call `a0.method(...)` is then emitted as

```rust
ptr.with_mut(|__v: <rule param type>| __v.method(...))
```

For example, the `push_back` rule is written as an ordinary `&mut` method call:

```rust
fn f21<T1: Clone>(a0: &mut Vec<T1>, a1: T1) { ... a0.push(...) }
```

Given the C++ input `v.push_back(20);` where `v` is reached through a
`Ptr<Vec<i32>>`, the generated code is:

```rust
v.with_mut(|__v: &mut Vec<i32>| __v.push(20));
```

When the receiver is a plain local value rather than a pointer, condition 3
fails and no closure is emitted; the same rule produces a direct call like
`(*v2.borrow_mut()).push(0);`.

The rewrite applies to pointer dereferences (`p->push_back(20)`) and to
reference usages (`r.push_back(20)` with `std::vector<int> &r = *p`); both are
[translated as a `Ptr`](../codegen/pointers.md), and that `Ptr` is what
`with_mut` is called on.

## Borrowing for the whole call

A rule that uses a `&mut` parameter more than once, such as `append` above,
would borrow the pointee once for each use. When the argument is reached through
a pointer, the converter instead borrows it once around the whole body and
substitutes the borrow for every use of the parameter:

```rust
r.with_mut(|__b0| {
    (*__b0).pop();
    Ptr::<i8>::from_string_literal(b"y")
        .with_slice(1_usize, |__s| (*__b0).extend_from_slice(__s));
    (*__b0).push(0);
});
```

## Evaluating arguments first

While a rule borrows an argument mutably through a `RefCell`, whether through a
`with_mut` closure, a `borrow_mut()` guard, or for the whole call, the other
arguments it evaluates must not borrow the same value, or the program panics.
C++ evaluates the arguments before the call anyway, so the refcount converter
binds them first, as it does with arguments that have side effects. Exempt are
those that borrow nothing, constants and the values of local scalars and
pointers, and objects the rule only reads, which would otherwise be copied just
to be read. For `s.append(2, s[0])` on a boxed `s`:

```rust
match ({ { (*s.borrow())[0_usize] } },) {
    (mut __tmp_0,) => {
        (*s.borrow_mut()).pop();
        let __len = (*s.borrow()).len() + 2_usize;
        (*s.borrow_mut()).resize(__len, __tmp_0);
        (*s.borrow_mut()).push(0);
    }
};
```

## The read-access counterpart

For read access the converter does not emit a closure. A pointer receiver whose
rule parameter is a value or `&` type is simply dereferenced (`p.read()` or
`(*p.upgrade().deref())`); conversely, if the rule declares a `Ptr` parameter
but the argument is not a pointer, the converter inserts an `as_pointer()` cast
or [materializes a temporary](../codegen/temporaries.md).

## Preprocessor-side rewrites

Two rewrites in `rule-preprocessor` exist to make the `with_mut` rewrite
possible. Both apply only to `&mut` parameters:

- A `*` deref in front of the parameter is dropped from the body, since the
  substituted argument is already an lvalue or pointer expression.
- `std::mem::take(&mut aN)` collapses to a bare placeholder, so the converter
  can re-express the move against the actual argument (for a pointer that
  becomes `std::mem::take(&mut <lvalue>)` on the borrowed pointee). The spelling
  must be exactly this fully qualified form: `mem::take` or an imported `take`
  is not rewritten. The collapsed placeholder's access is left `unknown` in
  phase 1; phase 2 resolves the `std::mem::take` call to a move.
