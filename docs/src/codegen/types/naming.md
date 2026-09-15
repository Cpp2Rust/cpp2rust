# Naming

Rust has one flat namespace per module and no overloading, so C++ names are
flattened and disambiguated when they are emitted.

Records and enums are named by `Mapper::ToRustName` from their qualified C++
spelling: `::`, `<`, `>`, commas, and spaces all become `_`. So `ns::Foo` is
`ns_Foo`, the instantiation `MyContainer<int>` is `MyContainer_int_`, and a
struct `Level1` nested in `Level0` is `Level0_Level1`. The same name is used for
the struct, its `impl` blocks, and every mention of the type.

An anonymous struct, union, or enum is named `anon_N`, numbered in order of
first appearance. In C, an anonymous tag that is only reachable through a
typedef (`typedef struct { ... } Point;`) is emitted as `Point_struct` (or
`Point_enum`), because C keeps tags and ordinary identifiers in separate
namespaces and `Point` may already be a variable or function.

Names that are Rust keywords get a trailing underscore: a variable `type`
becomes `type_`. The same applies to a keyword followed only by underscores, so
a C++ identifier that was already `type_` becomes `type__` and cannot collide
with the renamed `type`.

Free functions and global variables get a numeric suffix (`main_0`, `foo_3`)
from a process-wide table keyed by mangled name, which keeps overloads and
same-named `static` functions from different files apart. Methods keep their
name unless they are overloaded, in which case the parameter types are appended
(`method_i32`, `method_i32_const`). `operator<` is emitted as `lt`; comparison
operators additionally produce the corresponding trait impls (`PartialOrd`,
`Ord`, `PartialEq`).
