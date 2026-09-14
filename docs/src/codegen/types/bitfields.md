# Bit-fields

Bit-fields are not implemented. `VisitFieldDecl` ignores the declared width, so
a field such as `unsigned flags : 3;` is emitted as a plain `u32` field: the
struct compiles, but its layout, size, and the wrap-around of the field's value
differ from C.

> [!WARNING]
>
> Code that relies on bit-field layout or width (packing several fields into one
> word, `sizeof` on such a struct, storing a value wider than the field)
> translates silently to something else
> ([#312](https://github.com/Cpp2Rust/cpp2rust/issues/312)).
