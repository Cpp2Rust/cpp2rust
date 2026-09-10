# Iterators

A C++ iterator is a pointer-like object: it is dereferenced, compared against
`end()`, and moved with `++` and `--`, and it stays usable across the statements
of a loop body. Rust iterators are consumed by a `for` loop and cannot be
compared or stepped backwards, so the runtime represents C++ iterators with
values of its own.

## Random access iterators

For `std::vector`, `std::string`, and arrays the iterator is a
[`Ptr<T>`](./rc.md) into the container's buffer: `begin()` is `as_pointer()`,
`end()` is `to_end()`, and comparison and arithmetic are the pointer's own.
`Ptr<T>` also implements `Iterator`, yielding a pointer to each element, so a
range-based `for` becomes a Rust `for` over the pointer:

```cpp
std::vector<int> v;
for (auto x : v)
  printf("%d\n", x);
```

```rust
let v: Value<Vec<i32>> = Rc::new(RefCell::new(Vec::new()));
for x in v.as_pointer() as Ptr<i32> {
    println!("{}", x.read());
}
```

Two variants serve special cases. `StringIterator`, returned by
`to_string_iterator`, stops before the trailing zero byte, so iterating a
`std::string` visits its characters only. `PtrValueIter` yields copies of the
elements instead of pointers to them; rule bodies use it to feed a range of C
memory to Rust iterator adaptors:

```rust
// std::accumulate(first, last, init)
let count = (last - first) as usize;
PtrValueIter::new(&first, count).fold(init, |acc, x| acc + x)
```

## Stable iterators

`std::map<K, V>` is translated as a `BTreeMap<K, Value<V>>`, which has no
addressable elements to point into. The runtime defines `MapIter` for it: a pair
of a handle to the map and the current key, with `None` standing for `end()`.
Because it stores a key rather than a position, it survives insertions and
removals elsewhere in the map, as C++ guarantees. `begin`, `end`, and `find_key`
construct one; `inc` and `dec` move to the neighbouring key; `erase` removes the
current entry and returns the iterator to the next; the `++`/`--` traits and
`Iterator` are implemented on top of these. Two iterators compare equal when
they hold the same key, so `it != m.end()` compares `Some(key)` against `None`:

```cpp
std::map<int, double> m;
for (const auto &i : m)
  sum += i.second;
```

```rust
let m: Value<BTreeMap<i32, Value<f64>>> =
    Rc::new(RefCell::new(BTreeMap::new()));
for i in RefcountMapIter::begin(m.as_pointer()) {
    (*sum.borrow_mut()) += (*i.second().borrow());
}
```

> [!WARNING]
>
> Equality only looks at the key: iterators into two different maps compare
> equal when they hold the same key. In C++ comparing them is undefined
> behaviour, so the translation should panic with `ub:` instead; this will be
> fixed by comparing the map handles as well.

`first()` and `second()` come from the `MapIterator` trait and take the place of
`it->first` and `it->second`. `MapIter` is generic over how the map is reached,
which is what gives it an implementation for both models:
`RefcountMapIter<K, V>` holds a `Ptr<BTreeMap<K, Value<V>>>` and returns
`Value<K>` and `Value<V>`; `UnsafeMapIterator<K, V>` holds a
`*const BTreeMap<K, Box<V>>` and returns `*const K` and `*mut V`.
