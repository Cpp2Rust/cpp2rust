# Type Mappings

The table gives the spelling of each C++ type in both models, before any
refcount boxing. `T` stands for the translated inner type.

| C++                            | Unsafe model                                    | Refcount model                                            |
| ------------------------------ | ----------------------------------------------- | --------------------------------------------------------- |
| `bool`                         | `bool`                                          | `bool`                                                    |
| `int`, `unsigned long`, ...    | `i32`, `u64`, ... (host width)                  | same                                                      |
| `float`, `double`              | `f32`, `f64`                                    | same                                                      |
| `char`                         | `libc::c_char`                                  | `u8`                                                      |
| `size_t` and other typedefs    | by type rule (`usize`), else desugared          | same                                                      |
| `T[N]`                         | `[T; N]`                                        | `Box<[T]>`                                                |
| `T[]`                          | `[T]`                                           | `Box<[T]>`                                                |
| `struct S`, `enum E`           | `S`, `E`                                        | same                                                      |
| `T *`, `T &`                   | `*mut T`, `*const T`                            | [`Ptr<T>`](../../runtime/rc.md#values-and-pointers)       |
| `Abstract *`                   | `*mut dyn Abstract`                             | [`PtrDyn<dyn Abstract>`](../../runtime/ptr-dyn.md)        |
| `void *`                       | `*mut ::libc::c_void`                           | [`AnyPtr`](../../runtime/void.md)                         |
| `R (*)(A)`                     | `Option<unsafe fn(A) -> R>`                     | [`FnPtr<fn(A) -> R>`](../../runtime/fn-ptr.md)            |
| `va_list`                      | [`VaList`](../../runtime/va-args.md)            | [`VaList`](../../runtime/va-args.md)                      |
| lambda closure                 | `impl Fn(A) -> R` as a parameter, `_` elsewhere | same                                                      |
| `std::unique_ptr<T>`           | by type rule (`Option<Box<T>>`)                 | by type rule (`Option<Value<T>>`)                         |
| `std::vector<T>` and other STL | by type rule (`Vec<T>`)                         | by type rule (`Vec<T>`, `Vec<Value<Vec<T>>>` when nested) |

Other built-ins (`wchar_t`, `long double`, `char16_t`) are omitted. Rvalue
references (`T &&`) have no mapping of their own; they reach the converter only
through `std::move` and implicit move constructors, which are handled by rules
and by the constructor translation.

## User-defined types as rules

When a record or enum declaration is converted,
`Mapper::AddRuleForUserDefinedType` registers it in the mapper's type table: the
C++ name maps to the Rust name, and its pointer form maps to `*mut Name` or
`Ptr<Name>` (`*mut dyn Name` or `PtrDyn<dyn Name>` for abstract classes); nested
records are registered too. This is what makes library types instantiated with
user types translatable: `Mapper::Map` matches `std::vector<Item>` against the
rule for `std::vector<T1>` and then has to map `T1 = Item` through the same
table, which would fail if `Item` were not in it.

## Scalars

`char` is `libc::c_char` in the unsafe model, whose signedness follows the
platform like C's, and `u8` in the refcount model, because C strings are byte
vectors there (see [C Strings](../../runtime/cstr.md)). Since most C
implementations have signed `char`, the refcount model is set to switch to `i8`
([#246](https://github.com/Cpp2Rust/cpp2rust/issues/246)).

## Arrays

In the refcount model a constant array becomes `Box<[T]>`, dropping the length.
`Ptr<T>` carries only the element type, not `N`, so a `[T; N]` could not be
pointed to without a `Ptr` per length; `Box<[T]>` gives arrays of every length,
and heap arrays, the same shape. `[T; N]` survives only inside `sizeof`, which
becomes `::std::mem::size_of::<[T; N]>()`.

Array parameters decay to pointers as in C.

## Typedefs and qualifiers

Typedef names are looked up as type rules before being desugared, which is how
`size_t` maps to `usize` instead of the underlying `unsigned long`.

Constness is dropped: in the unsafe model it survives only as `*const` on
pointers and as a missing `mut` on bindings, and in the refcount model it has no
representation.

The [Pointers and References](./pointers.md) page covers how values of pointer
types are read and written.
