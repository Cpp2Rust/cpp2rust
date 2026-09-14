# Special-cased Library Types

Library types are translated by [type rules](../../rules/overview.md), and for
most of them the converter does nothing beyond applying the rule. A few types
also have code of their own in the converter, which decides how their values are
dereferenced, iterated, or initialized. Some of it could be moved into
[rules](../../rules/writing-rules.md). This page lists what the converter does
today; the rest of the type comes from its rule module under `rules/`.

## `std::unique_ptr`

The type rule maps `std::unique_ptr<T>` to `Option<Box<T>>` in the unsafe model
and `Option<Value<T>>` in the refcount model, and `std::make_unique` and
`std::move` are rules too. What the converter special-cases (`IsUniquePtr` in
`converter_lib`) is everything that treats a `unique_ptr` as a pointer:

Given

```cpp
std::unique_ptr<int> x1 = std::make_unique<int>(0);
std::unique_ptr<int> x2 = std::make_unique<int>(0);
*x2 = 1;
x1 = std::move(x2);
int *raw = &*x1;
```

the unsafe model produces

```rust
let mut x1: Option<Box<i32>> = Some(Box::new(0));
let mut x2: Option<Box<i32>> = Some(Box::new(0));
*x2.as_deref_mut().unwrap() = 1;
x1 = x2;
let mut raw: *mut i32 = &mut (*x1.as_deref_mut().unwrap()) as *mut i32;
```

and the refcount model produces

```rust
let x1: Value<Option<Value<i32>>> =
    Rc::new(RefCell::new(Some(Rc::new(RefCell::new(0)))));
let x2: Value<Option<Value<i32>>> =
    Rc::new(RefCell::new(Some(Rc::new(RefCell::new(0)))));
*(*x2.borrow_mut()).as_ref().unwrap().borrow_mut() = 1;
*x1.borrow_mut() = (*x2.borrow_mut()).take();
let raw: Value<Ptr<i32>> =
    Rc::new(RefCell::new((*x1.borrow()).as_pointer()));
```

`*p` and `p->x` are overloaded operator calls in C++; the converter emits the
`as_deref_mut().unwrap()` and `as_ref().unwrap().borrow_mut()` forms instead of
an operator call, `&*p` becomes the raw pointer or `as_pointer()`, and
`std::move` of a `unique_ptr` is a plain move or a `take()`. In the unsafe model
`p == nullptr` becomes `p.is_none()`; the refcount model does not special-case
it and emits `is_null()` as for any pointer, which no test exercises on an
`Option<Value<T>>`. A struct with a `unique_ptr` field does not derive `Copy`.

## Iterators

Iterator types come from rules (`std::vector<T>::iterator` maps to `*mut T` or
`Ptr<T>`, `std::map<K, V>::iterator` to `UnsafeMapIterator<K, V>` or
`RefcountMapIter<K, V>`), and the converter classifies them by
`GetStrongestIteratorCategory`: rule types marked as refcount pointers are
contiguous iterators and are handled exactly like a `Ptr<T>`, and the map
iterator types are bidirectional. The classification drives a few decisions.

Given

```cpp
std::map<int, double> m;
double sum = 0;
for (const auto &i : m) {
  sum += i.second;
}
auto it = m.begin();
sum += it->second;
```

the unsafe model produces

```rust
for i in UnsafeMapIterator::begin(&m as *const BTreeMap<i32, Box<f64>>) {
    sum += *i.second();
}
let mut it: UnsafeMapIterator<i32, f64> =
    UnsafeMapIterator::begin(&m as *const BTreeMap<i32, Box<f64>>);
sum += *it.second();
```

and the refcount model produces

```rust
for i in RefcountMapIter::begin(m.as_pointer()) {
    *sum.borrow_mut() += *i.second().borrow();
}
let it: Value<RefcountMapIter<i32, f64>> =
    Rc::new(RefCell::new(RefcountMapIter::begin(m.as_pointer())));
*sum.borrow_mut() += *(*it.borrow()).second().borrow();
```

`it->second` on a bidirectional iterator (map iterator) is not a pointer
dereference plus a field access, since the map iterator types have no pointer to
hand out; the converter emits the iterator itself and the field rule turns the
access into an accessor call.

The loop variable of a range-`for` over a `std::map` is the map iterator itself,
an entry with `first()`/`second()` accessors, not a pointer to an element; the
converter remembers such variables in [`map_iter_decls_`](../internals/state.md)
so that uses of them are not dereferenced.

A converting-constructor call that only wraps an iterator does not clone it
(`PushSuppressIteratorClone`). libstdc++ and libc++ differ in whether such a
wrapping constructor appears in the AST, so skipping the clone keeps the output
identical on Linux and macOS.

`IsIteratorType` recognizes any record that declares an `iterator_category`
typedef.

## `std::array`

`std::array<T, N>` maps to `Vec<T>` (see `rules/array`), so an initializer
`{1, 2, 3}` becomes `vec![1, 2, 3]`. The converter knows the type by name in
three places: the default value of an uninitialized `std::array` variable is
built element by element from `N`, a struct with a `std::array` field does not
derive `Default`, and it does not derive `Copy` either since the field is a
`Vec`.

> [!WARNING]
>
> An empty initializer, `std::array<int, 3> a = {};`, becomes `vec![]`, a vector
> of length 0, where C++ value-initializes `N` elements; indexing it panics
> ([#313](https://github.com/Cpp2Rust/cpp2rust/issues/313)).

## `std::string` and streams

`std::string` maps to `Vec<libc::c_char>` in the unsafe model and `Vec<u8>` in
the refcount model through its rules; the converter itself only special-cases
string literals (their type in an initializer, and ASCII escaping) and
range-`for` over a string. `std::ostream` calls (`std::cout << x`) are detected
with `IsCallToOstream` and translated by a dedicated path rather than by rules;
that path is described with `printf` under Expressions.
