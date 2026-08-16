# Virtual Classes

A pointer to a virtual class cannot be a `Ptr<T>`. The class is translated as a
Rust trait, and trait objects are unsized, which Rust marks with `dyn`. The
runtime provides a dedicated `PtrDyn<dyn T>` type for these pointers, kept
separate so the generic `Ptr` pays no cost for dynamic dispatch. `to_strong`
upgrades a `Ptr<T>` into a `Value<T>`, and `as_pointer_dyn` turns a
`Value<dyn T>` into a `PtrDyn<dyn T>`; a virtual call upgrades the pointer and
dispatches through the trait.
