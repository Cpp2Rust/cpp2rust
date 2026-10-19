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
- `StackVec` and `HeapVec`: a growable buffer; `std::vector` contents and string
  literals live in one.
- `Nested`: an inner vector or array of a nested container, such as a row of
  `std::vector<std::vector<int>>` or of `int[2][3]`, or a field of a struct that
  is itself an element of one. The inner container is stored inline in the outer
  one, so the pointer holds the way to reach it, as an `Rc<dyn SliceParent<T>>`:
  the pointer to it, the `Value` and the field, or the pointer to the struct and
  the field (see
  [Pointers into array and vector fields](#pointers-into-array-and-vector-fields)).
  It reaches the inner container through the outer one on every access.
- `Field`: a field of a struct, or an element of an array or vector field of a
  struct (see [Pointers to fields](#pointers-to-fields)).
- `Reinterpreted`: a byte-level view produced by a cast (see
  [Type Reinterpretation](./reinterpret.md)).

An array carries one reference counter for the whole allocation, not one per
element: the pointer pairs a weak reference to the whole array with the offset
of the element it points to, which keeps the memory and performance overhead of
arrays low.

Two pointers compare equal when they point into the same allocation at the same
byte offset, and ordering compares allocation addresses, as C++ pointer
comparison does.

## Pointers to fields

Fields are stored inline in their struct, so a struct and all of its fields
share one allocation and one `RefCell`. A pointer to a field is to a struct what
a pointer to an element is to an array: it records a weak reference to the
allocation that holds the struct, its root, together with the byte offset of the
field in it. `field_ptr!` creates one:

```c
struct point { int x; int y; };
struct point p;
int *y = &p.y;
```

```rust
let p: Value<point> = Rc::new(RefCell::new(<point>::default()));
let y: Value<Ptr<i32>> = Rc::new(RefCell::new(field_ptr!(p, y)));
```

`field_ptr!(p, y)` works on a `Value` or a `Ptr` to a struct. The offsets are
those of the C layout of the struct, which the code generator gets from Clang
and writes on the fields as `#[offset(N)]` attributes (any constant expression
works, e.g., `offset_of!` in the [libc shims](./libc-shims.md));
`#[derive(Record)]` turns them into the implementation of the `Record` trait:

```rust
#[derive(Clone, Record, Default)]
pub struct point {
    #[offset(0)]
    pub x: i32,
    #[offset(4)]
    pub y: i32,
}
```

Creating a field pointer allocates nothing. The byte offset of the field is kept
in the pointer's offset, which for a field pointer counts bytes, as for a
reinterpreted pointer. Pointers to fields of fields, and to fields of elements
of arrays in `Value`s, add up the offsets, and keep the allocation of the
outermost struct or array as their root.

An access through a field pointer borrows the root, and finds the field from its
offset and its type with `Record::locate`, which the derive generates as a
comparison of the offset against those of the fields, recursing into nested
structs. A field is identified by its type too, as a struct and its first field
share an offset. An offset that doesn't find a field of the pointer's type
panics with `ub: invalid field pointer`.

Arrays and vectors are stored inline too, so a field pointer can point to a
whole array or vector field, but it always points to a single object.

### Pointers into array and vector fields

A pointer to an element of an array or vector field reaches the array or vector
through the struct's allocation on every access. When the pointer is made from a
pointer to a struct in a `Value` (a variable, an element of an array or vector
variable, or a heap allocation), whose type is known, it is a `Nested` pointer
whose parent holds a weak reference to the `Value` and a closure that projects
its contents to the field. The array or vector is then reached without looking
the field up, nor checking its type.

Otherwise, the struct is reached through a field pointer, whose root has lost
its type, and the pointer is a `Field` pointer too, with the shape `Array` or
`Vec`: it holds the byte offset of the field, in the padding of the tag of
`PtrKind`, and the pointer's offset is the index of the element. Creating one
allocates nothing, pointer arithmetic changes the index alone, and the length of
the array or vector is found anew on every use, as the vector may grow.
`Record::len_at` gives it from the offset of the field, without knowing its
type.

`array_field_ptr!(p, arr)` is a pointer to element 0 of the array field `arr`.
When `p` is a reinterpreted pointer, the array lives in the bytes of the
original allocation, so the result is a reinterpreted pointer to those bytes, at
the offset of the field. Writes to the elements hence reach the original
allocation, and the pointer can go past the end of the struct, as C code does
with a trailing `char name[1]` in an over-allocated struct. A vector field
decays with `field_ref!(p, v).as_pointer()`, which is a pointer either to the
whole vector or to its first element, depending on the type asked for.

A struct that is an element of an array or vector field, or of a nested
container, has no byte offset in the outermost struct. A pointer to one of its
fields, or into one of its array or vector fields, is hence a `Nested` pointer,
whose parent holds the pointer to the struct and the closures that project the
struct to the field, which `field_ptr!`, `array_field_ptr!` and `field_ref!`
pass. These pointers allocate their parent once, and access the field through
the pointer to the struct without looking it up.

An element that is only read or written, as in `p->arr[i] = x` or
`y = p->v[i].z`, needs no pointer: `array_elem!(p, arr, i)` and
`vec_elem!(p, v, i)` are its place, which is accessed in a closure that borrows
the struct, `p.with(|s| f(&s.arr[i]))`, as `field!` does for a field.
`array_elem!` goes through `array_field_ptr!` instead when `p` is reinterpreted.
An element of an array that is itself reached through a place, such as a row of
a two-dimensional array field, is `index!(place, i)`.

Reading a field through a pointer borrows the struct only for the duration of a
closure, e.g., `p.with(|s| s.y)`, so that the borrow ends before the rest of the
statement runs. `field!(p, y)` is the place of the field, which is read and
written through `p`, projected to the field by a closure, without looking the
field up: `field!(p, y).write(3)`. It holds a reference to `p`, and takes no
more room than a pointer. `field_ptr!` is only used where an actual pointer to
the field is needed.

A field of a reinterpreted struct is itself a reinterpreted pointer, to the
bytes of the field. Conversely, reinterpreting a field pointer views the bytes
of the field alone, as if it were its own allocation.

Since all fields share the borrow of their struct, an expression must not write
to a field while another field of the same struct is borrowed. The code
generator scopes the borrows of field reads so that they end before any write in
the same statement.

## The heap

`new` and `new[]` are translated as `Ptr::alloc` and `Ptr::alloc_array`, and
`malloc`, `calloc`, `realloc`, and `strdup` allocate through `Ptr::alloc_array`
as well; the `alloc` module defines them as named functions (`malloc_refcount`,
`free_refcount`, `realloc_refcount`, `calloc_refcount`, `strdup_refcount`, and
their `_unsafe` twins for the unsafe model). The allocation's `Rc` is leaked
with `Rc::into_raw` so the object outlives the statement that created it, and
`delete` and `delete_array` recover the leaked reference and drop it:

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
or `Vec` pointer panics with `ub:`.

A heap allocation can also change hands instead of being freed. `to_owned_opt`
recovers the leaked reference the same way `delete` does, but returns it to the
caller as an owning `Option<Value<T>>` (or `Option<Value<Box<[T]>>>` for an
array), with `None` for the null pointer; from then on the allocation lives
exactly as long as that binding. It panics for stack, `Vec`, and reinterpreted
pointers. This is how `std::unique_ptr<T>` is translated: the smart pointer is
an `Option<Value<T>>`, its constructor and `reset` adopt a raw pointer with
`to_owned_opt`, and `as_pointer`, which is also implemented for
`Option<Value<T>>` and yields null for `None`, stands in for `get()`:

```cpp
std::unique_ptr<int> u(new int(1));
int *raw = u.get();
```

```rust
let u: Value<Option<Value<i32>>> =
    Rc::new(RefCell::new(Ptr::alloc(1).to_owned_opt()));
let raw: Value<Ptr<i32>> = Rc::new(RefCell::new((*u.borrow()).as_pointer()));
```

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

`with_slice` and `with_slice_mut` are the same idea over a range of elements:
they expose `len` bytes starting at the pointer as a Rust slice for the duration
of a closure, which is how a C buffer is passed to Rust and nix functions such
as `read` and `write`.

In every case the `RefCell` is borrowed only for the duration of the access,
which is what lets freely aliasing C++ pointers coexist with the borrow checker:
no borrow outlives the expression that created it. When an expression needs an
actual Rust reference, the pointer is upgraded to a
[`StrongPtr`](#strong-pointers), which holds the allocation alive and hands out
a `Ref`.

These borrows are the model's mutability checks, moved from compile time to run
time. Rust's rule still holds, any number of readers or one writer, but it is
enforced when the access happens: an expression that writes a variable while
also reading it through an alias, such as `*x.borrow_mut() = *x.borrow() + 1`,
traps. The code generator is responsible for not emitting such expressions: it
stores intermediate results in temporaries, so the reading borrow ends before
the writing borrow starts.

## Strong pointers

`upgrade` turns a `Ptr<T>` into a `StrongPtr<T>`, the same pointer holding a
strong `Rc` to its allocation instead of a weak one:

```rust
pub enum StrongPtr<T> {
    StackSingle(Rc<RefCell<T>>),
    Vec { rc: Rc<RefCell<Vec<T>>>, offset: usize },
    StackArray { rc: Rc<RefCell<Box<[T]>>>, offset: usize },
    Field { root: Rc<dyn Root>, offset: usize },
    Reinterpreted { alloc: OriginalAlloc, byte_offset: usize, cell: RefCell<Option<T>> },
}
```

`deref` returns a `Ref<'_, T>` to the pointee. The `Ref` borrows the
`StrongPtr`, so the borrow of the `RefCell` lasts as long as the strong pointer
does: in `p.upgrade().deref().field`, the temporary `StrongPtr` lives until the
end of the enclosing statement, and so does the borrow. The code generator
prefers the `with` and `with_mut` closures, and upgrades only where the result
of an access must borrow the pointee beyond a closure. There is no `deref_mut`;
writes go through `write` and `with_mut`, including those to a field of a struct
reached through a pointer: `field!(p, x).write(v)`.

For the `Reinterpreted` variant there is no value to reference, only bytes in
another allocation. `deref` reads those bytes into a local cell and hands out a
`Ref` to that copy, refreshing it on every call. `with_mut` writes the bytes
through to the original allocation before it returns, so that a write is visible
through every other pointer at once.

> [!WARNING]
>
> `StrongPtr` is set to be removed. Holding a strong reference, even briefly,
> undermines the model in two ways:
>
> 1. Nothing prevents a `StrongPtr` from outliving its statement. One that is
>    stored, returned, or bound to a local keeps the object alive past the point
>    where C++ destroys it, so its destructor runs late and dangling accesses go
>    unnoticed; a heap object held this way makes the later `delete` panic. The
>    generator only ever emits it as a temporary, but hand-edited code or a rule
>    can break that.
> 2. Even as a temporary, it lives for the whole statement. In
>    `(*p.upgrade().deref()).method()` the strong reference is alive during the
>    call, so a method that runs `delete this`, or otherwise deletes the object
>    it was called on, hits `delete`'s reference-count check and panics with a
>    spurious `ub: invalid delete`.

Where the code generator needs a different view of the same allocation, it does
not upgrade at all. `decay` turns a pointer to a whole `Vec<T>` or `Box<[T]>`
into a pointer to its first element by re-tagging the existing weak reference;
when the `Vec<T>` or `Box<[T]>` is itself an element of another container, it
instead allocates a `Nested` pointer that holds the pointer to it. `Ptr::to_dyn`
(see [Virtual Classes](./ptr-dyn.md)) re-tags the weak reference in the same way
for the upcast to a trait object.

## Arithmetic

The offset lives in the pointer, so arithmetic never touches the allocation.
`p + n`, `p - n`, and the `++`/`--` forms move the offset, including past the
end of the allocation, exactly as C++ allows; bounds are checked only when the
pointer is dereferenced. Subtracting two pointers yields their element distance
and requires both to point into the same allocation.

The pointer also knows the extent of its allocation: `len` is the number of
elements in it, whatever the pointer's offset, and `get_offset` is the pointer's
element index within it. The container rules build `end()` and `back()` pointers
from these (`to_end`, `to_last`) and turn a `[first, last)` range into a count
or an absolute index the same way.

## Integer casts

Casts between pointers and integers are translated as `to_int` and `from_int`:

```c
uintptr_t n = (uintptr_t)p;
int *q = (int *)n;
```

```rust
let n: Value<usize> = Rc::new(RefCell::new((*p.borrow()).to_int()));
let q: Value<Ptr<i32>> =
    Rc::new(RefCell::new(<Ptr<i32>>::from_int(*n.borrow())));
```

Both currently panic when executed. Giving them well-defined semantics is work
in progress ([#225](https://github.com/Cpp2Rust/cpp2rust/pull/225)).
