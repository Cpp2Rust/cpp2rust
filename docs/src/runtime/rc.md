# Reference Counting

The refcount model produces safe Rust, and `rc.rs` is where that safety comes
from. It defines the two types every translated program is built on: `Value<T>`,
the translation of a C++ variable, and `Ptr<T>`, the translation of a C++
pointer.

## Values and pointers

Rust requires every value to have a single owner, known at compile time, and
references to follow the borrow rules. C++ promises neither: a variable can be
aliased by any number of pointers, and any of them may write. Proving ownership
in the presence of such unrestricted aliasing is undecidable in general, so the
refcount model does not try. Instead it moves Rust's ownership and mutability
checks from compile time to run time, trading some speed for safety: `Rc` counts
references and checks lifetimes dynamically, and `RefCell` checks at each access
that readers and writers do not overlap.

A C++ variable is therefore translated as a `Value<T>`, an alias for
`Rc<RefCell<T>>`. Taking the address of a variable becomes a call to
`as_pointer`, which produces a `Ptr<T>`:

```c
int b = 2;
int *b_ptr = &b;
*b_ptr = 3;
```

```rust
let b: Value<i32> = Rc::new(RefCell::new(2));
let b_ptr: Value<Ptr<i32>> = Rc::new(RefCell::new(b.as_pointer()));
(*b_ptr.borrow()).write(3);
```

## Weak references

A C++ pointer does not own what it points to, and `Ptr<T>` keeps that property:
it holds a `Weak` reference to the allocation plus an element offset. Ownership
stays with the variable binding for stack values and with the allocation itself
for the heap. When the owner goes away, every pointer into it dangles, and the
next access panics instead of reading freed memory.

The choice of weak over strong references is about destructors. C++ RAII code
relies on destructors running at precise points, such as a mutex being released
at the end of a scope; a strong reference held by a stray pointer could keep the
object alive past that point and run its destructor late. With weak references,
objects die exactly where C++ says they do, and a pointer that outlives its
object dangles.

This is the central property of the model: memory bugs of the original program,
such as use after free, double free, and null or out-of-bounds dereference,
become panics in the translated one. Their messages carry the `ub:` prefix.

## Pointer kinds

A `Ptr<T>` knows what it points into:

- `Null`: the null pointer, and the default value.
- `StackSingle` and `HeapSingle`: a single value.
- `StackArray` and `HeapArray`: a fixed-size array.
- `Vec`: a growable buffer; `std::vector` contents and string literals live in
  one.
- `Reinterpreted`: a byte-level view produced by a cast (see
  [Type Reinterpretation](./reinterpret.md)).

An array carries one reference counter for the whole allocation, not one per
element: the pointer pairs a weak reference to the whole array with the offset
of the element it points to, which keeps the memory and performance overhead of
arrays low.

Two pointers compare equal when they point into the same allocation at the same
byte offset, and ordering compares allocation addresses, as C++ pointer
comparison does.

## The heap

`new` and `new[]` are translated as `Ptr::alloc` and `Ptr::alloc_array`, and
`malloc`, `calloc`, and `realloc` allocate through `Ptr::alloc_array` as well.
The allocation's `Rc` is deliberately leaked so the object outlives the
statement that created it. The leak is legitimate: a Rust program that leaks
memory is still well typed. `delete` and `delete_array` recover the leaked
reference and drop it:

```c
int *d = new int(0);
*d = 5;
delete d;
```

```rust
let d: Value<Ptr<i32>> = Rc::new(RefCell::new(Ptr::alloc(0)));
(*d.borrow()).write(5);
(*d.borrow()).delete();
```

`delete` checks that the pointer still points at the start of a live heap
allocation: freeing twice, freeing through an offset pointer, or freeing a stack
value panics with `ub:`.

## Dereferences

A dereference becomes a short-lived borrow. `read` and `write` copy a value out
of or into the allocation:

```c
*d = 5;
int v = *d;
```

```rust
(*d.borrow()).write(5);
let v: Value<i32> = Rc::new(RefCell::new((*d.borrow()).read()));
```

A `Ptr` cannot simply return a `&T` or `&mut T` to its pointee: the reference
would keep the `RefCell` borrowed with nothing to bound its lifetime. `with` and
`with_mut` invert the control instead: the expression that needs the reference
moves into a closure, and the borrow lasts exactly as long as the closure runs.
They carry the operations that need a reference to the existing value, such as a
`push_back` on a vector reached through a pointer (`write` could only replace
the vector wholesale):

```rust
v.with_mut(|v| v.push(20));
```

Applied rule bodies are the main producer of these calls (see
[Rule Rewriting](../rules/rewriting.md)). `write` itself is a thin wrapper: it
is defined as `with_mut(|v| *v = value)`.

In every case the `RefCell` is borrowed only for the duration of the access,
which is what lets freely aliasing C++ pointers coexist with the borrow checker:
no borrow outlives the expression that created it. When an expression needs an
actual Rust reference, the pointer is upgraded to a
[`StrongPtr`](../codegen/pointers.md), which holds the allocation alive and
hands out a `Ref`.

These borrows are the model's mutability checks, moved from compile time to run
time. Rust's rule still holds, any number of readers or one writer, but it is
enforced when the access happens: an expression that writes a variable while
also reading it through an alias, such as `*x.borrow_mut() = *x.borrow() + 1`,
traps. The code generator is responsible for not emitting such expressions: it
stores intermediate results in temporaries, so the reading borrow ends before
the writing borrow starts.

## Arithmetic

The offset lives in the pointer, so arithmetic never touches the allocation.
`p + n`, `p - n`, and the `++`/`--` forms move the offset, including past the
end of the allocation, exactly as C++ allows; bounds are checked only when the
pointer is dereferenced. Subtracting two pointers yields their element distance
and requires both to point into the same allocation.

## Integer casts

Casts between pointers and integers are translated as `to_int` and `from_int`:

```c
uintptr_t n = (uintptr_t)p;
int *q = (int *)n;
```

```rust
let n: Value<usize> = Rc::new(RefCell::new((*p.borrow()).to_int()));
let q: Value<Ptr<i32>> = Rc::new(RefCell::new(<Ptr<i32>>::from_int(*n.borrow())));
```

Both currently panic when executed. Giving them well-defined semantics is work
in progress.
