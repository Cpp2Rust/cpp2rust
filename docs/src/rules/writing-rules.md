# Writing Rules

This page shows how to write rules for each kind of C++ construct. In every case
the recipe is the same: write an `fN` (or `tN`) function on the C++ side whose
single `return` statement exercises the construct, and a same-named function on
the Rust side giving the translation.

## Free functions

```cpp
// rules/stat/src.cpp
int f1(const char *pathname, struct stat *statbuf) {
  return stat(pathname, statbuf);
}
```

```rust
// rules/stat/tgt_refcount.rs
fn f1(a0: Ptr<u8>, a1: Ptr<Stat>) -> i32 {
    match nix::sys::stat::stat(a0.to_rust_string().as_str()) {
        Ok(__s) => {
            a1.with_mut(|__st| *__st = Stat::from_libc(&__s));
            0
        }
        Err(__e) => {
            libcc2rs::cpp2rust_errno().write(__e as i32);
            -1
        }
    }
}
```

Rule bodies may be arbitrarily complex; multi-statement bodies are wrapped in a
block when spliced into the output.

`return` statements are prohibited in Rust rule bodies (the preprocessor rejects
them); produce the result as a tail expression instead. The body is not emitted
as a function of its own: it is spliced inline into the generated code as a
block expression, so a `return` would not end the rule, it would return from
whatever generated function the rule happens to be expanded in.

When the pattern's type cannot be named, the rule uses an `auto` return type:
`rules/iomanip` writes `auto f1(int n) { return std::setw(n); }` because
`std::setw` returns an unspecified type.

## Methods

There is no special syntax for member functions: write a free function that
takes the receiver as its first parameter and calls the method on it. On the
Rust side the receiver is `a0`.

```cpp
// rules/vector/src.cpp
template <typename T1> std::size_t f2(const std::vector<T1> &o) {
  return o.size();
}
```

```rust
// rules/vector/tgt_unsafe.rs
unsafe fn f2<T1>(a0: Vec<T1>) -> usize {
    a0.len()
}
```

Template rules use generic parameters named `T1`, `T2`, ... on both sides,
matched positionally. The rule is written against the open template
`std::vector<T1>`, with `T1` left as a placeholder, so a single rule covers
every instantiation: when the input program calls `size()` on, say, a
`std::vector<int>`, the matcher binds `T1 = int`.

## Static member functions

A static member function is also written with a receiver parameter, which exists
only to name the class. The call site has no receiver argument, so the Rust side
drops it and numbers the remaining parameters from `a0`; here there are none:

```cpp
// rules/limits/src.cpp
template <typename T1> T1 f1(std::numeric_limits<T1> &a0) { return a0.max(); }
```

```rust
// rules/limits/tgt_unsafe.rs
unsafe fn f1<T1: HasMinMax>() -> T1 {
    <T1>::MAX
}
```

(`HasMinMax` is a helper trait defined alongside the rules in the same file.)

## Constructors

Constructors are functions returning the type by value, one rule per overload:

```cpp
// rules/string/src.cpp
std::string f7(const char *s, std::size_t n) { return std::string(s, n); }
std::string f9(std::size_t n, char ch) { return std::string(n, ch); }
```

Overloads that differ in value category are distinct rules too: `rules/vector`
has separate rules for `push_back(const T1 &)` and `push_back(T1 &&)`.

No destructor rules exist so far: the STL and libc APIs covered by the current
rules have not needed any, since their types map to Rust types whose `Drop`
implementations already do the right thing.

## Operators

Write operators with explicit `operator` call syntax, in member form
(`x.operator@(...)`) or free form (`operator@(a, b)`):

```cpp
// rules/map/src.cpp
template <typename T1, typename T2>
T2 &f1(std::map<T1, T2> &o, const T1 &key) { return o.operator[](key); }

template <typename T1, typename T2>
bool f11(typename std::map<T1, T2>::iterator a,
         typename std::map<T1, T2>::iterator b) {
  return operator!=(a, b);
}
```

Post-increment is distinguished from pre-increment by the usual dummy `int`
parameter: `a0.operator++(a1)` versus `it.operator++()`. Conversion operators
use the same explicit syntax: `a0.operator T1 &()` in `rules/functional` matches
the conversion of a `std::reference_wrapper<T1>` back to a reference. Field
accesses are rules of their own, matched by the field: `it->first` and
`it->second` through iterators, plain `o.second` on a pair (`rules/map`,
`rules/pair`).

## Callable arguments

A rule parameter may be a callable. Function pointers are spelled directly; for
a lambda, whose type cannot be written, the rule declares a file-scope lambda
and takes `decltype(lambda)`:

```cpp
// rules/algorithm/src.cpp
auto lambda = [](const T2 &a, const T2 &b) { return false; };
void f6(T1 first, T1 last, decltype(lambda) comp) {
  return std::stable_sort(first, last, comp);
}
```

```rust
// rules/algorithm/tgt_unsafe.rs
unsafe fn f6<T1: Ord, T2>(a0: *mut T1, a1: *mut T1, a2: &mut T2)
where
    T2: FnMut(&T1, &T1) -> bool,
{ ... }
```

`T1` and `T2` are not template parameters here but file-scope helper structs
modelling an iterator and its value type; being named like generics, they bind
as `T1`/`T2` at the use site. The function pointer version of the comparator is
a separate rule (`f7`).

## Iterators

There is no iterator abstraction: an iterator type gets a type rule, and every
operation on it its own expression rule (`operator*`, `operator++`,
`operator!=`, ...). What the type maps to is up to the rule:
`std::string::iterator` becomes a plain pointer (`*mut libc::c_char` unsafe,
`Ptr<u8>` refcount), while `std::map` iterators become the runtime types
`libcc2rs::UnsafeMapIterator`/`MapIterator`. Dependent iterator types are named
with `typename`:

```cpp
// rules/map/src.cpp
template <typename T1, typename T2>
using t2 = typename std::map<T1, T2>::const_iterator;
```

## Types

A type rule has two halves. On the C++ side, declare a type alias named `tN` for
the C++ type being mapped. On the Rust side, write a function with the same name
that takes no arguments: its _return type_ is the Rust type that the C++ type
maps to, and its _body_ is the default value the generated code uses when it
needs to construct one (e.g. for an uninitialized variable). Reference and
pointer variants of a type each get their own rule:

```cpp
// rules/iostream/src.cpp
using t1 = std::ostream;
using t2 = std::ostream &;
using t3 = std::ostream *;
```

C structs use `typedef` instead of `using`:

```cpp
// rules/stat/src.cpp
typedef struct stat t1;
```

```rust
// rules/stat/tgt_unsafe.rs
fn t1() -> ::libc::stat { unsafe { std::mem::zeroed() } }
```

```rust
// rules/stat/tgt_refcount.rs
fn t1() -> libcc2rs::Stat { Default::default() }
```

A type rule may map to the sentinel type `libcc2rs::IgnoreRule`, meaning "this
model has no special mapping for the type"; the converter then falls back to its
normal type conversion. This is useful when only one model needs a custom
mapping: `rules/carray` maps multi-dimensional C arrays to nested boxed slices
in the refcount model, while its `tgt_unsafe.rs` targets are `IgnoreRule` so the
unsafe model keeps the default array conversion.

## Enum values, constants, and macros

Constants are `fN` functions that take no arguments and return the constant, one
rule per value:

```cpp
// rules/fcntl/src.cpp
int f3(void) { return O_CREAT; }
int f4(void) { return O_TRUNC; }
```

```rust
// rules/fcntl/tgt_unsafe.rs
unsafe fn f3() -> i32 { ::libc::O_CREAT }
```

For macros that expand to integer literals, the preprocessor records the _macro
name_ rather than the value, so `O_CREAT` in the input matches this rule by
name. Enum constants and global variables (e.g. `std::cout`) are matched by
their qualified name. A global and its address are separate rules:
`rules/iostream` maps both `std::cout` (`f1`) and `&std::cout` (`f3`).

Integer-literal macros are the only macros matchable directly. Macros whose
expansions are platform internals with no stable callee, such as `errno` or
`FD_SET`, are first rewritten into calls to synthetic `cpp2rust_*` functions by
the [compat shims](./compat.md); rules then match the shim call.

## Variadic functions

The C++ side uses a template parameter pack rather than a C-style `...`
parameter, out of necessity: a function that takes `...` cannot forward its
variadic arguments to another call, so a rule like

```cpp
int f1(int a0, int a1, ...) { return fcntl(a0, a1, ...); }
```

is not expressible. A parameter pack can be forwarded (`args...`), which is
exactly what the rule body needs to do. The Rust side takes a trailing parameter
that must be typed `&[VaArg]` and named `va`:

```cpp
// rules/fcntl/src.cpp
template <typename... Args>
int f1(int a0, int a1, Args... args) {
  return fcntl(a0, a1, args...);
}
```

```rust
// rules/fcntl/tgt_refcount.rs
fn f1(a0: i32, a1: i32, va: &[VaArg]) -> i32 { ... }
```

Bodies read the arguments through the va-args API in `libcc2rs` (`VaArg`,
`VaList`, the `VaArgGet` accessors, `format_c`).

## Passthrough rules

When a call should be forwarded verbatim to the same-named function in Rust's
`libc` crate, the Rust target can be an `extern` declaration instead of a body:

```cpp
// rules/fcntl/src.cpp
template <typename... Args>
int f1(int a0, int a1, Args... args) {
  return fcntl(a0, a1, args...);
}
```

```rust
// rules/fcntl/tgt_unsafe.rs
unsafe extern "C" {
    fn f1(a0: i32, a1: i32, ...) -> i32;
}
```

The converter then emits a direct `libc::fcntl(...)` call at the call site.

## Platform-specific rules

Gate the C++ side with the usual preprocessor conditionals and the Rust side
with `#[cfg(...)]`; the two must agree so that the rule name sets line up:

```cpp
// rules/socket/src.c
#ifdef __linux__
int f4(void) { return SOCK_CLOEXEC; }
#endif
```

```rust
// rules/socket/tgt_unsafe.rs
#[cfg(target_os = "linux")]
unsafe fn f4() -> i32 {
    libc::SOCK_CLOEXEC
}
```

The Rust preprocessor evaluates `#[cfg]` attributes against the host target
(only `target_os = linux|macos` and `target_arch = x86_64|x86` are accepted) and
drops non-matching rules.

Mutually exclusive platform branches use `#elif` with disjoint rule numbers:
`rules/errno` defines `f91` to `f135` under `__linux__` and `f136` to `f153`
under `__APPLE__`. Feature-test macros a pattern needs must come before the
includes, as with `#define _GNU_SOURCE` in `rules/socket/src.c`.

## Pattern resolution limits

The preprocessor resolves a template pattern by instantiating its template
parameters with synthesized types
([The Rule Preprocessors](./preprocessors.md#cpp-rule-preprocessor)):

- A bare `T1` becomes an empty struct, so the pattern cannot use members,
  operators, or nested types of `T1`.
- A parameter pack instantiates to the empty pack.
- A non-type parameter is pinned to the value `1`.

Unqualified callees are looked up in `namespace std` first and in the global
scope only when `std` has no match, so an unqualified name that exists in both
resolves to the `std` one.
