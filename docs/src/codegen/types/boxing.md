# Boxing

In the refcount model a variable is boxed: its type `T` is wrapped in
`Value<T>`, an alias for `Rc<RefCell<T>>` (see
[Reference Counting](../../runtime/rc.md)). Without the box, taking the address
of a variable would need a Rust reference, and arbitrary C++ aliasing cannot be
expressed with references.

Not every type position is boxed. `ConverterRefCount` keeps a stack of
conversion kinds, [`conversion_kind_`](../internals/state.md), and the construct
that owns the type pushes one before printing it:

- `FullRefCount`: pushed by variable declarations; `Convert(QualType)` wraps the
  result in `Value<...>`.
- `Pointee`: pushed by field declarations; the bare type is printed, also for
  arrays and types that map to a `Vec` or a `Box` (`std::vector`, `std::string`,
  `std::array`) (see below).
- `Unboxed`: pushed by parameter lists, return types, and record names; the bare
  type is printed.
- `Ptr`: pushed by a pointer type for its pointee; also printed bare.

The result by position:

| Position                                    | `int`        | `Item`        | `int[3]`             |
| ------------------------------------------- | ------------ | ------------- | -------------------- |
| local variable, global                      | `Value<i32>` | `Value<Item>` | `Value<Box<[i32]>>`  |
| function parameter, return type             | `i32`        | `Item`        | decays to `Ptr<i32>` |
| struct field                                | `i32`        | `Item`        | `Box<[i32]>`         |
| pointee of `Ptr<T>`, element of a container | `i32`        | `Item`        | `Box<[i32]>`         |

Parameters arrive unboxed and are re-boxed by the function preamble; return
values are unboxed:

```cpp
int add(int a, Item item) { return a + item.id; }
```

```rust
pub fn add_0(a: i32, item: Item) -> i32 {
    let a: Value<i32> = Rc::new(RefCell::new(a));
    let item: Value<Item> = Rc::new(RefCell::new(item));
    return *a.borrow() + (*item.borrow()).id;
}
```

C++ passes arguments to functions by copy, so signatures stay unboxed; boxing
the copy on entry then lets the body treat parameters exactly like local
variables. The preamble skips reference parameters, which are a `Ptr<T>` and
never boxed.

Nested containers, library ones and arrays alike, store their inner levels
inline: `std::vector<std::vector<int>>` maps to `Vec<Vec<i32>>`, and
`int a[2][2]` to `Box<[Box<[i32]>]>`, both inside the outer `Value<...>` of the
declaration. Copying a nested container with `clone` hence copies every level,
as C++ does. A pointer into an inner container is a `Nested` pointer (see
[Pointer kinds](../../runtime/rc.md#pointer-kinds)), which reaches the inner
container through the outer one.

Struct fields are stored inline, arrays and vectors included, so that a whole
struct is a single allocation, and a pointer to a field records the struct's
allocation and the field's byte offset (see [Pointers](pointers.md)). A pointer
to an element of an array or vector field, or to a field of such an element,
reaches it through the struct (see
[Pointers into array and vector fields](../../runtime/rc.md#pointers-into-array-and-vector-fields)),
and pointer arithmetic moves between the elements as for any other array:

```cpp
struct Holder { std::vector<Point> points; int n; };
h.points[0].y = 5;
p->points[1].y = 6;
```

```rust
pub struct Holder {
    #[offset(0)]
    #[byte_size(24)]
    pub points: Vec<Point>,
    #[offset(24)]
    pub n: i32,
}
(*h.borrow_mut()).points[(0_usize) as usize].y = 5;
field!(vec_elem!(p, points, 1_usize), y).write(6);
```

An array or vector field of a struct in a `Value` is accessed through the
struct's own `borrow()` or `borrow_mut()`. Through a pointer, `vec_elem!` and
`array_elem!` are the places of its elements, which are read and written in a
closure that borrows the struct, without making a pointer to the element. As
nothing in a struct is a `Value`, structs derive `Clone`, which copies their
arrays and vectors.
