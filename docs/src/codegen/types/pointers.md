# Pointers and References

The unsafe model keeps C++ pointers as raw pointers and dereferences them
directly. The refcount model replaces every pointer and reference with
[`Ptr<T>`](../../runtime/rc.md#values-and-pointers), a weak reference plus an
offset, and every dereference with a short-lived borrow of the pointee. Given

```cpp
int f(int *q) {
  int b = 2;
  int *p = &b;
  *p = *q;
  return b;
}
```

the unsafe model produces

```rust
pub unsafe fn f_0(mut q: *mut i32) -> i32 {
    let mut b: i32 = 2;
    let mut p: *mut i32 = &mut b as *mut i32;
    *p = *q;
    return b;
}
```

and the refcount model produces

```rust
pub fn f_0(q: Ptr<i32>) -> i32 {
    let q: Value<Ptr<i32>> = Rc::new(RefCell::new(q));
    let b: Value<i32> = Rc::new(RefCell::new(2));
    let p: Value<Ptr<i32>> = Rc::new(RefCell::new(b.as_pointer()));
    p.borrow().write(q.borrow().read());
    return *b.borrow();
}
```

The rest of the page goes through the pointer operations one at a time.

## Address-of

Unsafe model: `&x` becomes `&mut x as *mut T`, or `&x as *const T` when the
pointer type is to `const`. Globals use `&raw mut x` so no reference to the
`static` is formed. An array decays with `arr.as_mut_ptr()`, and the address of
an element is `&mut arr[i] as *mut T`.

Refcount model: `&x` becomes `x.as_pointer()`, which produces a `Ptr` holding a
weak reference to the variable's `Value`. `&s.field` is `field_ptr!(s, field)`,
and `&p->field` is `field_ptr!(p, field)`: a pointer to the field of the struct,
made from the `Value` or the `Ptr` of the struct (see
[Pointers to fields](../../runtime/rc.md#pointers-to-fields)). An array decays
with `arr.as_pointer() as Ptr<T>`, a `Ptr` to element 0 of the whole array, and
`&arr[i]` is that pointer offset by `i`.

Both models push the address down to the innermost place expression:
`&(cond ? x : y)` becomes `if cond { &mut x } else { &mut y }` in the unsafe
model and `if cond { x.as_pointer() } else { y.as_pointer() }` in the refcount
one. The `if` yields a value copied out of whichever branch ran, not that
branch's storage, so the address has to be taken inside the branches, where the
place is still known.

## Dereference

Unsafe model: `*p` stays `*p`, `p->x` becomes `(*p).x`, and an assignment
through a pointer is `*p = v`.

When the dereference is the base of an index, `(*p)[i]` with `p: *mut Vec<i32>`,
Rust would have to create a `&mut Vec<i32>` out of the raw pointer to call
`Index::index`, and its `dangerous_implicit_autorefs` lint rejects that as an
error. The converter makes the reference explicit instead: `EmitDeref` prints
`(&mut (*p))[i]`, or `(&(*p))[i]` when the `operator[]` is `const`, when
[`autoref_mut_`](../internals/state.md) is set. `PushExplicitAutoref` sets it
around the base of an overloaded subscript, which also covers a member of the
pointee, `(&mut (*hp)).v[i]`, around the range of a range-`for`, and around a
rule placeholder marked `is_index_base` (see [Rules IR](../../rules/ir.md));
`EmitDeref` clears it once used, so nested dereferences inside the base are
printed plainly.

Refcount model: a dereference cannot hand out a `&T` into the pointee, because
nothing would bound the borrow's lifetime, so a read copies the value out and a
write copies it in. Which form is emitted follows the
[expression kind](../expressions/kinds.md), what the enclosing construct expects
of the dereference:

- An rvalue use copies the value out. A scalar or pointer pointee is `p.read()`,
  and so is a whole record, `*p`. A field of a record pointee is read in a
  closure that borrows the record for its duration: `p->x` is
  `p.with(|__s: &S| __s.x)`, and `p->a[i].y` is `p.with(|__s: &S| __s.a[i].y)`.
  A method called on a field runs in the closure too, together with the rest of
  its rule: `p->v.size()` is `p.with(|__s: &S| (*__s.v.borrow()).len())`.
- A write to a field of a record pointee is also done in a closure, which
  borrows the record mutably: `p->x = v` is
  `p.with_mut(|__s: &mut S| __s.x = v)`. Operands that may access memory are
  evaluated before the closure (`let __rhs = ...;`), so that they don't find the
  record borrowed. Fields that are [`Value`s of their own](boxing.md) only need
  the record borrowed immutably,
  `p.with(|__s: &S| (*__s.v.borrow_mut()).push(1))`. The same closures are used
  for references to records, `r.x`, and for elements of arrays of records,
  `a[i].x`.
- An address-of use prints `p` itself.
- An lvalue use prints nothing at once. The converter records the pointer
  expression as a [pending dereference](../expressions/pending-deref.md), and
  whoever consumes the lvalue, an assignment or a mapped method call, wraps it:
  `p.write(v)`, or [`with_mut`](../../rules/rewriting.md) for a mutating method
  on a boxed pointee. This is what lets `*p = v` come out as a single `write`
  instead of a borrow followed by an assignment, and `*p += v` as
  `{ let _ptr = p.clone(); _ptr.write(_ptr.read() + v) }`.

`with` and `with_mut` work on any pointer, including a
[reinterpreted](../../runtime/reinterpret.md) one, whose pointee is decoded from
the bytes of the original allocation and, for `with_mut`, encoded back into them
before the closure returns. A write through a reinterpreted pointer is hence
visible right away through every other pointer to the same bytes.

## Arithmetic and comparison

Unsafe model: `p + n` is `p.offset(n as isize)` and `p - n` is
`p.offset(-(n as isize))`; `p - q` is
`(p as usize - q as usize) / ::std::mem::size_of::<T>()`; `++p` is
`p.prefix_inc()` through the [increment traits](../../runtime/inc-dec.md);
`p == NULL` is `p.is_null()` and the null literal is `std::ptr::null_mut()`, or
`std::ptr::null()` for a pointer to `const`.

Refcount model: the same operations on `Ptr`, `p.offset(n as isize)`,
`p.clone() - q.clone()` (subtraction takes its operands by value, hence the
clones), `p.prefix_inc()`, `p.is_null()`, and `Ptr::null()`. Arithmetic only
moves the offset; whether the result is in bounds is checked when it is
dereferenced.

## References

A C++ reference is a pointer that cannot be made to point elsewhere, and both
models translate it as one. In the unsafe model a reference parameter is
`*mut T` (`*const T` for `const T &`), an argument `f(x)` is
`f(&mut x as *mut T)`, and uses of the reference are `*r`. In the refcount model
it is a `Ptr<T>` that is [never boxed](./boxing.md): the argument is
`f(x.as_pointer())`, uses are `r.read()` and `r.write(v)`, and returning a
reference returns the `Ptr` (with a `.clone()`, since `Ptr` is not `Copy`).

## Heap

`new T(v)` becomes `Box::leak(Box::new(v)) as *mut T` in the unsafe model and
[`Ptr::alloc(v)`](../../runtime/rc.md#the-heap) in the refcount model;
`delete p` becomes `::std::mem::drop(Box::from_raw(p))` and `p.delete()`. Array
forms use a boxed slice and `Ptr::alloc_array`. In the refcount model the heap
allocation is a leaked `Rc` that `delete` recovers, so a double `delete` or a
`delete` of something that was not allocated with `new` panics instead of
corrupting memory.
