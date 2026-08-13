# Writing Rules

This page shows how to write rules for each kind of C++ construct. In every
case the recipe is the same: write an `fN` (or `tN`) function on the C++ side
whose single `return` statement exercises the construct, and a same-named
function on the Rust side giving the translation.

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

Rule bodies may be arbitrarily complex; multi-statement bodies are wrapped in
a block when spliced into the output.

`return` statements are prohibited in Rust rule bodies (the preprocessor
rejects them); produce the result as a tail expression instead. The body is
not emitted as a function of its own: it is spliced inline into the generated
code as a block expression, so a `return` would not end the rule, it would
return from whatever generated function the rule happens to be expanded in.

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

## Constructors

Constructors are functions returning the type by value, one rule per overload:

```cpp
// rules/string/src.cpp
std::string f7(const char *s, std::size_t n) { return std::string(s, n); }
std::string f9(std::size_t n, char ch) { return std::string(n, ch); }
```

Overloads that differ in value category are distinct rules too: `rules/vector`
has separate rules for `push_back(const T1 &)` and `push_back(T1 &&)`.

No destructor rules exist so far: the STL and libc APIs covered by the
current rules have not needed any, since their types map to Rust types whose
`Drop` implementations already do the right thing.

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
parameter: `a0.operator++(a1)` versus `it.operator++()`. Member accesses
through iterators are also rules (e.g. `it->first`, `it->second`, `o.second`
in `rules/map` and `rules/pair`).

## Types

A type rule has two halves. On the C++ side, declare a type alias named `tN`
for the C++ type being mapped. On the Rust side, write a function with the
same name that takes no arguments: its *return type* is the Rust type that
the C++ type maps to, and its *body* is the default value the generated code
uses when it needs to construct one (e.g. for an uninitialized variable).
Reference and pointer variants of a type each get their own rule:

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

A type rule may map to the sentinel type `libcc2rs::IgnoreRule`, meaning
"this model has no special mapping for the type"; the converter then falls
back to its normal type conversion. This is useful when only one model needs
a custom mapping: `rules/carray` maps multi-dimensional C arrays to nested
boxed slices in the refcount model, while its `tgt_unsafe.rs` targets are
`IgnoreRule` so the unsafe model keeps the default array conversion.

## Enum values, constants, and macros

Constants are `fN` functions that take no arguments and return the constant,
one rule per value:

```cpp
// rules/fcntl/src.cpp
int f3(void) { return O_CREAT; }
int f4(void) { return O_TRUNC; }
```

```rust
// rules/fcntl/tgt_unsafe.rs
unsafe fn f3() -> i32 { ::libc::O_CREAT }
```

For macros that expand to integer literals, the preprocessor records the
*macro name* rather than the value, so `O_CREAT` in the input matches this
rule by name. Enum constants and global variables (e.g. `std::cout`) are
matched by their qualified name.

## Variadic functions

The C++ side uses a template parameter pack rather than a C-style `...`
parameter, out of necessity: a function that takes `...` cannot forward its
variadic arguments to another call, so a rule like

```cpp
int f1(int a0, int a1, ...) { return fcntl(a0, a1, ...); }
```

is not
expressible. A parameter pack can be forwarded (`args...`), which is exactly
what the rule body needs to do. The Rust side takes a trailing parameter that
must be typed `&[VaArg]` and named `va`:

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

## Passthrough rules

When a call should be forwarded verbatim to the same-named function in Rust's
`libc` crate, the Rust target can be an `extern` declaration instead of a
body:

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
(only `target_os = linux|macos` and `target_arch = x86_64|x86` are accepted)
and drops non-matching rules.
