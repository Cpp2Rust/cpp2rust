# Boxing

In the refcount model a variable is boxed: its type `T` is wrapped in
`Value<T>`, an alias for `Rc<RefCell<T>>` (see
[Reference Counting](../../runtime/rc.md)). Without the box, taking the address
of a variable would need a Rust reference, and arbitrary C++ aliasing cannot be
expressed with references.

Not every type position is boxed. `ConverterRefCount` keeps a stack of
conversion kinds, [`conversion_kind_`](../internals/state.md), and the construct
that owns the type pushes one before printing it:

- `FullRefCount`: pushed by variable and field declarations; `Convert(QualType)`
  wraps the result in `Value<...>`.
- `Unboxed`: pushed by parameter lists, return types, and record names; the bare
  type is printed.
- `Ptr`: pushed by a pointer type for its pointee; also printed bare.

The result by position:

| Position                                    | `int`        | `Item`        | `int[3]`             |
| ------------------------------------------- | ------------ | ------------- | -------------------- |
| local variable, struct field, global        | `Value<i32>` | `Value<Item>` | `Value<Box<[i32]>>`  |
| function parameter, return type             | `i32`        | `Item`        | decays to `Ptr<i32>` |
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
    return *a.borrow() + *(*item.borrow()).id.borrow();
}
```

C++ passes arguments to functions by copy, so signatures stay unboxed; boxing
the copy on entry then lets the body treat parameters exactly like local
variables. The preamble skips reference parameters, which are a `Ptr<T>` and
never boxed.

Nested containers, library ones and arrays alike, box each level except the
innermost, so that every inner container can be borrowed and mutated on its own,
and a pointer can be taken to it. The boxing is written into the type rules
themselves: `std::vector<std::vector<int>>` maps to `Vec<Value<Vec<i32>>>`, and
the `carray` rules map `int a[2][2]` to `Box<[Value<Box<[i32]>>]>`, both before
the outer `Value<...>` of the declaration is added.
