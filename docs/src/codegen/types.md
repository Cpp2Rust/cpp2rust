# Types

Every place the converter prints a type goes through `Convert(QualType)`. It
first asks the [type rules](../rules/writing-rules.md) for a mapping, so library
types and typedef names such as `size_t` are resolved by rules, and only falls
back to the `Visit*Type` methods for the built-in and user-defined types
described here.

Given

```cpp
struct Item {
  int id;
  char name[8];
  std::vector<int> refs;
};

int count(Item item) { return item.id; }
```

the unsafe model produces (attributes and trait impls omitted)

```rust
pub struct Item {
    pub id: i32,
    pub name: [libc::c_char; 8],
    pub refs: Vec<i32>,
}
pub unsafe fn count_0(mut item: Item) -> i32 {
    return item.id;
}
```

and the refcount model produces

```rust
pub struct Item {
    pub id: Value<i32>,
    pub name: Value<Box<[u8]>>,
    pub refs: Value<Vec<i32>>,
}
pub fn count_0(item: Item) -> i32 {
    let item: Value<Item> = Rc::new(RefCell::new(item));
    return *(*item.borrow()).id.borrow();
}
```

Every struct field is boxed in its own `Value<T>` so that a pointer can be taken
to it. This is set to change: field writes and field addresses through a
reinterpreted struct pointer go to a temporary and are lost, so fields will
become plain `T` and `Ptr` will gain a kind that stores the parent struct plus
an offset ([#309](https://github.com/Cpp2Rust/cpp2rust/issues/309)).
